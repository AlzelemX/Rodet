// A small Language Server Protocol client: JSON-RPC over the backend's
// `lsp.*` requests and events. The backend only frames the messages.
import { backend, errorText } from '../backend';
import { Emitter } from '../util';

export interface LspPosition {
  line: number;
  character: number;
}
export interface LspRange {
  start: LspPosition;
  end: LspPosition;
}
export interface LspLocation {
  uri: string;
  range: LspRange;
}
export interface LspDiagnostic {
  range: LspRange;
  severity?: 1 | 2 | 3 | 4;
  code?: string | number;
  codeDescription?: { href: string };
  source?: string;
  message: string;
  tags?: number[];
  relatedInformation?: { location: LspLocation; message: string }[];
  data?: unknown;
}
export interface LspTextEdit {
  range: LspRange;
  newText: string;
}
export interface LspWorkspaceEdit {
  changes?: Record<string, LspTextEdit[]>;
  documentChanges?: any[];
}

type RequestHandler = (params: any) => any | Promise<any>;

export type ServerState = 'stopped' | 'starting' | 'indexing' | 'ready' | 'error' | 'missing';

export class LspClient {
  private nextId = 1;
  private pending = new Map<number, { resolve: (v: any) => void; reject: (e: Error) => void; method: string }>();
  private notificationHandlers = new Map<string, Set<(params: any) => void>>();
  private requestHandlers = new Map<string, RequestHandler>();
  private generation = 0;
  private unlisten: (() => void)[] = [];

  capabilities: any = {};
  state: ServerState = 'stopped';
  readonly onState = new Emitter<ServerState>();
  readonly onLog = new Emitter<string>();
  /** Fired when the server restarts, so documents are opened again. */
  readonly onReady = new Emitter<void>();
  serverPath = '';
  lastError = '';

  constructor() {
    this.unlisten.push(
      backend.on('lsp.message', ({ generation, message }) => {
        if (generation !== this.generation) return;
        this.receive(JSON.parse(message));
      }),
      backend.on('lsp.log', ({ line }) => this.onLog.fire(line)),
      backend.on('lsp.exit', ({ generation }) => {
        if (generation !== this.generation) return;
        for (const p of this.pending.values()) p.reject(new Error('the language server stopped'));
        this.pending.clear();
        if (this.state !== 'stopped') this.setState('error');
      }),
    );
  }

  private setState(state: ServerState) {
    this.state = state;
    this.onState.fire(state);
  }

  async start(root: string, initializationOptions: unknown, rootUri: string) {
    this.setState('starting');
    // Nothing meant for the previous server goes to the new one.
    this.outbox = [];
    try {
      const started = await backend.call<{ generation: number; server: string }>('lsp.start', { root });
      this.generation = started.generation;
      this.serverPath = started.server;
    } catch (err) {
      this.lastError = errorText(err);
      this.setState(this.lastError.includes('not found') ? 'missing' : 'error');
      return false;
    }
    const result = await this.request('initialize', {
      processId: null,
      clientInfo: { name: 'DEVR+', version: '0.1.0' },
      rootUri,
      workspaceFolders: [{ uri: rootUri, name: root.split(/[\\/]/).pop() }],
      initializationOptions,
      capabilities: clientCapabilities,
    });
    this.capabilities = result.capabilities ?? {};
    this.notify('initialized', {});
    this.setState('indexing');
    this.onReady.fire();
    return true;
  }

  async stop() {
    if (this.state === 'stopped') return;
    this.setState('stopped');
    try {
      await Promise.race([this.request('shutdown', null), new Promise((r) => setTimeout(r, 800))]);
      this.notify('exit', null);
    } catch {
      // Stopping anyway.
    }
    await backend.call('lsp.stop').catch(() => {});
  }

  get running() {
    return this.state === 'indexing' || this.state === 'ready';
  }

  markReady(ready: boolean) {
    if (!this.running) return;
    this.setState(ready ? 'ready' : 'indexing');
  }

  request<T = any>(method: string, params: unknown, signal?: { onCancellationRequested?: (cb: () => void) => void }): Promise<T> {
    const id = this.nextId++;
    const promise = new Promise<T>((resolve, reject) => {
      this.pending.set(id, { resolve, reject, method });
    });
    this.send({ jsonrpc: '2.0', id, method, params });
    signal?.onCancellationRequested?.(() => {
      if (this.pending.has(id)) this.notify('$/cancelRequest', { id });
    });
    return promise;
  }

  notify(method: string, params: unknown) {
    this.send({ jsonrpc: '2.0', method, params });
  }

  onNotification(method: string, handler: (params: any) => void) {
    let set = this.notificationHandlers.get(method);
    if (!set) this.notificationHandlers.set(method, (set = new Set()));
    set.add(handler);
  }

  onRequest(method: string, handler: RequestHandler) {
    this.requestHandlers.set(method, handler);
  }

  private outbox: string[] = [];
  private sending = false;

  /** Queues a message. One backend call at a time carries all queued
   * messages, so the server gets them in the order they were sent: an edit
   * always arrives before the completion request that follows it. */
  private send(message: unknown) {
    this.outbox.push(JSON.stringify(message));
    if (!this.sending) this.drain();
  }

  private async drain() {
    this.sending = true;
    while (this.outbox.length) {
      const messages = this.outbox.splice(0);
      try {
        await backend.call('lsp.send', { messages });
      } catch (err) {
        this.onLog.fire(`DEVR+: can't send to the language server: ${errorText(err)}`);
      }
    }
    this.sending = false;
  }

  private async receive(message: any) {
    if (message.id !== undefined && message.method === undefined) {
      const pending = this.pending.get(message.id);
      if (!pending) return;
      this.pending.delete(message.id);
      if (message.error) {
        const err = new Error(message.error.message ?? 'request failed');
        (err as any).code = message.error.code;
        pending.reject(err);
      } else {
        pending.resolve(message.result);
      }
      return;
    }
    if (message.id !== undefined) {
      const handler = this.requestHandlers.get(message.method);
      try {
        const result = handler ? await handler(message.params) : null;
        this.send({ jsonrpc: '2.0', id: message.id, result: result ?? null });
      } catch (err) {
        this.send({ jsonrpc: '2.0', id: message.id, error: { code: -32603, message: errorText(err) } });
      }
      return;
    }
    for (const handler of this.notificationHandlers.get(message.method) ?? []) handler(message.params);
  }
}

/** Request errors that only mean "the answer is out of date". */
export function isCancelled(err: unknown) {
  const code = (err as any)?.code;
  return code === -32800 || code === -32801;
}

const clientCapabilities = {
  general: { positionEncodings: ['utf-16'] },
  window: { workDoneProgress: true, showMessage: {} },
  workspace: {
    applyEdit: true,
    workspaceEdit: { documentChanges: true, resourceOperations: ['create', 'rename', 'delete'] },
    configuration: true,
    symbol: { symbolKind: { valueSet: Array.from({ length: 26 }, (_, i) => i + 1) } },
    didChangeWatchedFiles: { dynamicRegistration: false },
  },
  textDocument: {
    synchronization: { didSave: true, dynamicRegistration: false },
    completion: {
      completionItem: {
        snippetSupport: true,
        insertReplaceSupport: true,
        labelDetailsSupport: true,
        deprecatedSupport: true,
        documentationFormat: ['markdown', 'plaintext'],
        resolveSupport: { properties: ['documentation', 'detail', 'additionalTextEdits'] },
      },
      contextSupport: true,
    },
    hover: { contentFormat: ['markdown', 'plaintext'] },
    signatureHelp: {
      signatureInformation: {
        documentationFormat: ['markdown', 'plaintext'],
        parameterInformation: { labelOffsetSupport: true },
        activeParameterSupport: true,
      },
    },
    definition: { linkSupport: true },
    typeDefinition: { linkSupport: true },
    implementation: { linkSupport: true },
    references: {},
    documentHighlight: {},
    documentSymbol: { hierarchicalDocumentSymbolSupport: true },
    codeAction: {
      codeActionLiteralSupport: {
        codeActionKind: {
          valueSet: ['', 'quickfix', 'refactor', 'refactor.extract', 'refactor.inline', 'refactor.rewrite', 'source'],
        },
      },
      isPreferredSupport: true,
      dataSupport: true,
      resolveSupport: { properties: ['edit'] },
    },
    codeLens: {},
    formatting: {},
    rename: { prepareSupport: true },
    publishDiagnostics: { relatedInformation: true, tagSupport: { valueSet: [1, 2] }, codeDescriptionSupport: true },
    foldingRange: { lineFoldingOnly: true },
    selectionRange: {},
    inlayHint: { resolveSupport: { properties: ['tooltip', 'label.tooltip'] } },
    semanticTokens: {
      requests: { full: { delta: false }, range: false },
      tokenTypes: [
        'namespace', 'type', 'class', 'enum', 'interface', 'struct', 'typeParameter', 'parameter',
        'variable', 'property', 'enumMember', 'event', 'function', 'method', 'macro', 'keyword',
        'modifier', 'comment', 'string', 'number', 'regexp', 'operator', 'decorator',
      ],
      tokenModifiers: [
        'declaration', 'definition', 'readonly', 'static', 'deprecated', 'abstract', 'async',
        'modification', 'documentation', 'defaultLibrary',
      ],
      formats: ['relative'],
      multilineTokenSupport: false,
      overlappingTokenSupport: false,
    },
  },
  experimental: {
    serverStatusNotification: true,
    commands: { commands: ['rust-analyzer.runSingle', 'rust-analyzer.showReferences', 'rust-analyzer.gotoLocation'] },
  },
};
