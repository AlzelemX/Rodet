// Notifications (bottom right) and small dialogs (input, confirm).
import { h, icon } from '../util';

let stack: HTMLElement | null = null;

export type Level = 'info' | 'warning' | 'error' | 'success';

export interface Action {
  label: string;
  run: () => void;
  primary?: boolean;
}

export function toast(message: string, level: Level = 'info', actions: Action[] = [], timeout = 6000) {
  if (!stack) {
    stack = h('div.toasts');
    document.body.appendChild(stack);
  }
  const iconName = { info: 'info', warning: 'warning', error: 'error', success: 'pass' }[level];
  const close = () => {
    el.classList.add('leaving');
    setTimeout(() => el.remove(), 180);
  };
  const el = h(
    `div.toast.${level}` as 'div',
    {},
    icon(iconName, 'toast-icon'),
    h('div.toast-body', {}, h('div.toast-message', {}, message),
      actions.length
        ? h('div.toast-actions', {}, actions.map((a) =>
            h(a.primary ? 'button.btn.primary' : 'button.btn', { onclick: () => { close(); a.run(); } }, a.label)))
        : null),
    h('button.icon-button.toast-close', { title: 'Close', onclick: close }, icon('close')),
  );
  stack.appendChild(el);
  if (timeout > 0 && actions.length === 0) setTimeout(close, timeout);
  return close;
}

function modal(content: HTMLElement, onClose: () => void): () => void {
  const backdrop = h('div.modal-backdrop', {}, content);
  const close = () => {
    backdrop.remove();
    document.removeEventListener('keydown', onKey, true);
    onClose();
  };
  const onKey = (e: KeyboardEvent) => {
    if (e.key === 'Escape') {
      e.preventDefault();
      e.stopPropagation();
      close();
    }
  };
  backdrop.addEventListener('mousedown', (e) => {
    if (e.target === backdrop) close();
  });
  document.addEventListener('keydown', onKey, true);
  document.body.appendChild(backdrop);
  return close;
}

/** Asks for a line of text. Resolves `null` when cancelled. */
export function prompt(title: string, value = '', options: { placeholder?: string; select?: [number, number]; validate?: (v: string) => string | null } = {}): Promise<string | null> {
  return new Promise((resolve) => {
    let done = false;
    const finish = (result: string | null) => {
      if (done) return;
      done = true;
      close();
      resolve(result);
    };
    const error = h('div.dialog-error');
    const input = h('input.input', { value, placeholder: options.placeholder ?? '', spellcheck: false });
    const check = () => {
      const message = options.validate?.(input.value) ?? null;
      error.textContent = message ?? '';
      return message === null;
    };
    input.addEventListener('input', check);
    input.addEventListener('keydown', (e) => {
      if (e.key === 'Enter' && check()) finish(input.value);
    });
    const box = h('div.dialog', {}, h('div.dialog-title', {}, title), input, error,
      h('div.dialog-buttons', {},
        h('button.btn', { onclick: () => finish(null) }, 'Cancel'),
        h('button.btn.primary', { onclick: () => check() && finish(input.value) }, 'OK')));
    const close = modal(box, () => finish(null));
    setTimeout(() => {
      input.focus();
      const [start, end] = options.select ?? [0, value.length];
      input.setSelectionRange(start, end);
    });
  });
}

export function confirm(title: string, detail = '', okLabel = 'OK', danger = false): Promise<boolean> {
  return new Promise((resolve) => {
    let done = false;
    const finish = (result: boolean) => {
      if (done) return;
      done = true;
      close();
      resolve(result);
    };
    const ok = h(danger ? 'button.btn.danger' : 'button.btn.primary', { onclick: () => finish(true) }, okLabel);
    const box = h('div.dialog', {}, h('div.dialog-title', {}, title), detail ? h('div.dialog-detail', {}, detail) : null,
      h('div.dialog-buttons', {}, h('button.btn', { onclick: () => finish(false) }, 'Cancel'), ok));
    const close = modal(box, () => finish(false));
    setTimeout(() => ok.focus());
  });
}

/** A dialog with any content; returns its close function. */
export function showDialog(title: string, body: HTMLElement, wide = false): () => void {
  let close = () => {};
  const box = h(wide ? 'div.dialog.wide' : 'div.dialog', {},
    h('div.dialog-header', {}, h('div.dialog-title', {}, title),
      h('button.icon-button', { title: 'Close', onclick: () => close() }, icon('close'))),
    body);
  close = modal(box, () => {});
  return close;
}
