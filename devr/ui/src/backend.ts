// Talks to the DEVR+ backend (devr-core): inside the desktop app through
// Tauri, in a browser through devr-server's WebSocket. Both carry the same
// requests (`call`) and events (`on`).

type Listener = (payload: any) => void;

export interface Backend {
  readonly kind: 'desktop' | 'web';
  call<T = any>(method: string, params?: unknown): Promise<T>;
  on(event: string, listener: Listener): () => void;
}

class Listeners {
  private map = new Map<string, Set<Listener>>();
  on(event: string, listener: Listener): () => void {
    let set = this.map.get(event);
    if (!set) this.map.set(event, (set = new Set()));
    set.add(listener);
    return () => set!.delete(listener);
  }
  emit(event: string, payload: unknown) {
    for (const listener of this.map.get(event) ?? []) {
      try {
        listener(payload);
      } catch (err) {
        console.error(`DEVR+: ${event} listener failed`, err);
      }
    }
  }
}

class TauriBackend implements Backend {
  readonly kind = 'desktop' as const;
  private listeners = new Listeners();
  private ready: Promise<unknown>;
  private invoke: Promise<typeof import('@tauri-apps/api/core').invoke>;

  constructor() {
    this.invoke = import('@tauri-apps/api/core').then((m) => m.invoke);
    this.ready = import('@tauri-apps/api/event').then(({ listen }) =>
      listen<{ event: string; payload: unknown }>('devr-event', (e) =>
        this.listeners.emit(e.payload.event, e.payload.payload),
      ),
    );
  }

  async call<T>(method: string, params: unknown = {}): Promise<T> {
    await this.ready;
    const invoke = await this.invoke;
    return invoke<T>('devr_call', { method, params });
  }

  on(event: string, listener: Listener) {
    return this.listeners.on(event, listener);
  }
}

class WebBackend implements Backend {
  readonly kind = 'web' as const;
  private listeners = new Listeners();
  private socket!: WebSocket;
  private open!: Promise<void>;
  private nextId = 1;
  private pending = new Map<number, { resolve: (v: any) => void; reject: (e: Error) => void }>();

  constructor() {
    this.connect();
  }

  private connect() {
    const url = new URL('ws', location.href);
    url.protocol = location.protocol === 'https:' ? 'wss:' : 'ws:';
    this.socket = new WebSocket(url);
    this.open = new Promise((resolve, reject) => {
      this.socket.onopen = () => resolve();
      this.socket.onerror = () => reject(new Error('can’t reach devr-server'));
    });
    this.socket.onmessage = (e) => {
      const message = JSON.parse(e.data);
      if (message.event) {
        this.listeners.emit(message.event, message.payload);
        return;
      }
      const pending = this.pending.get(message.id);
      if (!pending) return;
      this.pending.delete(message.id);
      if (message.error !== undefined) pending.reject(new Error(message.error));
      else pending.resolve(message.result);
    };
    this.socket.onclose = () => {
      for (const p of this.pending.values()) p.reject(new Error('disconnected from devr-server'));
      this.pending.clear();
      this.listeners.emit('backend.disconnected', {});
    };
  }

  async call<T>(method: string, params: unknown = {}): Promise<T> {
    await this.open;
    const id = this.nextId++;
    return new Promise<T>((resolve, reject) => {
      this.pending.set(id, { resolve, reject });
      this.socket.send(JSON.stringify({ id, method, params }));
    });
  }

  on(event: string, listener: Listener) {
    return this.listeners.on(event, listener);
  }
}

export const backend: Backend =
  '__TAURI_INTERNALS__' in window ? new TauriBackend() : new WebBackend();

/** Error text without the `Error:` prefix Tauri and fetch add. */
export function errorText(err: unknown): string {
  const text = err instanceof Error ? err.message : String(err);
  return text.replace(/^Error:\s*/, '');
}
