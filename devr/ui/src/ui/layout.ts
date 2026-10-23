// The workbench skeleton: activity bar, side bar, editor area, bottom panel
// and status bar, with draggable borders like VS Code.
import { t } from '../i18n';
import { Emitter, clamp, h, icon } from '../util';

export interface SideView {
  id: string;
  title: string;
  icon: string;
  element: HTMLElement;
  actions?: HTMLElement[];
  onShow?: () => void;
  badge?: HTMLElement;
}

export interface PanelView {
  id: string;
  title: string;
  element: HTMLElement;
  count?: HTMLElement;
  actions?: HTMLElement[];
  onShow?: () => void;
}

class Layout {
  readonly root = h('div.workbench');
  readonly activitybar = h('nav.activitybar');
  readonly sidebar = h('aside.sidebar');
  readonly sidebarTitle = h('span.title');
  readonly sidebarActions = h('div', { style: { display: 'flex', gap: '2px' } });
  readonly sidebarBody = h('div.sidebar-body');
  readonly center = h('div.center');
  readonly editorGroup = h('section.editor-group');
  readonly tabsBar = h('div.tabs-bar');
  readonly breadcrumbs = h('div.breadcrumbs');
  readonly editorHost = h('div.editor-host');
  readonly panel = h('section.panel');
  readonly panelTabs = h('div', { style: { display: 'flex' } });
  readonly panelActions = h('div', { style: { display: 'flex', gap: '2px' } });
  readonly panelBody = h('div.panel-body');
  readonly statusbar = h('footer.statusbar');
  readonly statusLeft = h('div', { style: { display: 'flex', alignItems: 'center' } });
  readonly statusRight = h('div', { style: { display: 'flex', alignItems: 'center' } });
  readonly activityBottom = h('div', { style: { display: 'flex', flexDirection: 'column' } });

  private sideViews = new Map<string, { view: SideView; button: HTMLButtonElement }>();
  private panelViews = new Map<string, { view: PanelView; tab: HTMLButtonElement }>();
  activeSide: string | null = null;
  activePanel: string | null = null;
  readonly onResize = new Emitter<void>();

  mount(parent: HTMLElement) {
    const sidebarSash = h('div.sash');
    const panelSash = h('div.sash-h');
    this.sidebar.append(
      h('div.sidebar-header', {}, this.sidebarTitle, this.sidebarActions),
      this.sidebarBody,
    );
    this.activitybar.append(h('div.spacer'), this.activityBottom);
    this.editorGroup.append(this.tabsBar, this.breadcrumbs, this.editorHost);
    this.panel.append(
      h('div.panel-header', {}, this.panelTabs, h('div.spacer'), this.panelActions,
        h('button.icon-button', { title: t('Maximize Panel'), onclick: () => this.togglePanelMaximized() }, icon('chevron-up')),
        h('button.icon-button', { title: t('Close Panel (Ctrl+J)'), onclick: () => this.togglePanel(false) }, icon('close'))),
      this.panelBody,
    );
    this.center.append(this.editorGroup, panelSash, this.panel);
    this.statusbar.append(this.statusLeft, h('div.spacer'), this.statusRight);
    this.root.append(h('div.main', {}, this.activitybar, this.sidebar, sidebarSash, this.center), this.statusbar);
    parent.appendChild(this.root);

    this.restoreSizes();
    drag(sidebarSash, 'x', (dx, start) => {
      this.sidebar.style.width = `${clamp(start + dx, 170, window.innerWidth * 0.6)}px`;
      this.onResize.fire();
    }, () => this.sidebar.getBoundingClientRect().width, () => this.saveSizes());
    drag(panelSash, 'y', (dy, start) => {
      this.panel.style.height = `${clamp(start - dy, 90, this.center.getBoundingClientRect().height - 120)}px`;
      this.onResize.fire();
    }, () => this.panel.getBoundingClientRect().height, () => this.saveSizes());
    window.addEventListener('resize', () => this.onResize.fire());
  }

  addSideView(view: SideView, bottom = false) {
    const button = h('button.activity-item', { title: view.title, onclick: () => this.toggleSide(view.id) }, icon(view.icon));
    if (view.badge) button.appendChild(view.badge);
    if (bottom) this.activityBottom.appendChild(button);
    else this.activitybar.insertBefore(button, this.activitybar.querySelector('.spacer'));
    view.element.classList.add('view');
    this.sidebarBody.appendChild(view.element);
    this.sideViews.set(view.id, { view, button });
  }

  /** A button at the bottom of the activity bar (not a view). */
  addActivityButton(title: string, iconName: string, onclick: () => void) {
    this.activityBottom.appendChild(h('button.activity-item', { title, onclick }, icon(iconName)));
  }

  showSide(id: string) {
    const entry = this.sideViews.get(id);
    if (!entry) return;
    this.sidebar.classList.remove('hidden');
    for (const [otherId, other] of this.sideViews) {
      const active = otherId === id;
      other.view.element.classList.toggle('active', active);
      other.button.classList.toggle('active', active);
    }
    this.activeSide = id;
    this.sidebarTitle.textContent = entry.view.title;
    this.sidebarActions.replaceChildren(...(entry.view.actions ?? []));
    entry.view.onShow?.();
    this.onResize.fire();
  }

  toggleSide(id: string) {
    if (this.activeSide === id && !this.sidebar.classList.contains('hidden')) {
      this.sidebar.classList.add('hidden');
      this.sideViews.get(id)?.button.classList.remove('active');
      this.activeSide = null;
      this.onResize.fire();
    } else {
      this.showSide(id);
    }
  }

  toggleSidebar() {
    if (this.sidebar.classList.contains('hidden')) this.showSide(this.activeSide ?? 'explorer');
    else this.toggleSide(this.activeSide ?? 'explorer');
  }

  addPanelView(view: PanelView) {
    const tab = h('button.panel-tab', { onclick: () => this.showPanel(view.id) }, view.title, view.count ?? null);
    this.panelTabs.appendChild(tab);
    view.element.classList.add('panel-view');
    this.panelBody.appendChild(view.element);
    this.panelViews.set(view.id, { view, tab });
  }

  showPanel(id: string) {
    const entry = this.panelViews.get(id);
    if (!entry) return;
    this.panel.classList.remove('hidden');
    for (const [otherId, other] of this.panelViews) {
      const active = otherId === id;
      other.view.element.classList.toggle('active', active);
      other.tab.classList.toggle('active', active);
    }
    this.activePanel = id;
    this.panelActions.replaceChildren(...(entry.view.actions ?? []));
    entry.view.onShow?.();
    this.onResize.fire();
  }

  togglePanel(show?: boolean) {
    const visible = !this.panel.classList.contains('hidden');
    const want = show ?? !visible;
    if (want) this.showPanel(this.activePanel ?? 'terminal');
    else this.panel.classList.add('hidden');
    this.onResize.fire();
  }

  private maximized = false;
  private togglePanelMaximized() {
    this.maximized = !this.maximized;
    this.editorGroup.style.display = this.maximized ? 'none' : '';
    this.panel.style.flex = this.maximized ? '1' : '';
    this.onResize.fire();
  }

  private saveSizes() {
    try {
      localStorage.setItem('devr.layout', JSON.stringify({
        sidebar: this.sidebar.getBoundingClientRect().width,
        panel: this.panel.getBoundingClientRect().height,
      }));
    } catch {
      // Storage unavailable.
    }
  }

  private restoreSizes() {
    try {
      const saved = JSON.parse(localStorage.getItem('devr.layout') ?? '{}');
      if (saved.sidebar) this.sidebar.style.width = `${saved.sidebar}px`;
      if (saved.panel) this.panel.style.height = `${saved.panel}px`;
    } catch {
      // Defaults.
    }
  }
}

function drag(
  sash: HTMLElement,
  axis: 'x' | 'y',
  move: (delta: number, start: number) => void,
  startSize: () => number,
  done: () => void,
) {
  sash.addEventListener('pointerdown', (e) => {
    e.preventDefault();
    const origin = axis === 'x' ? e.clientX : e.clientY;
    const start = startSize();
    sash.classList.add('dragging');
    sash.setPointerCapture(e.pointerId);
    const onMove = (ev: PointerEvent) => move((axis === 'x' ? ev.clientX : ev.clientY) - origin, start);
    const onUp = () => {
      sash.classList.remove('dragging');
      sash.removeEventListener('pointermove', onMove);
      sash.removeEventListener('pointerup', onUp);
      done();
    };
    sash.addEventListener('pointermove', onMove);
    sash.addEventListener('pointerup', onUp);
  });
}

export const layout = new Layout();

// ---- context menus ----

export interface MenuItem {
  label: string;
  icon?: string;
  key?: string;
  run: () => void;
}

export function contextMenu(e: MouseEvent, items: (MenuItem | 'sep')[]) {
  e.preventDefault();
  document.querySelectorAll('.context-menu').forEach((m) => m.remove());
  const menu = h('div.context-menu', {},
    items.map((item) => item === 'sep'
      ? h('div.sep')
      : h('div.item', { onclick: () => { close(); item.run(); } },
          item.icon ? icon(item.icon) : h('span', { style: { width: '16px' } }),
          item.label,
          item.key ? h('span.key', {}, item.key) : null)));
  document.body.appendChild(menu);
  const rect = menu.getBoundingClientRect();
  menu.style.left = `${Math.min(e.clientX, window.innerWidth - rect.width - 4)}px`;
  menu.style.top = `${Math.min(e.clientY, window.innerHeight - rect.height - 4)}px`;
  const close = () => {
    menu.remove();
    document.removeEventListener('mousedown', outside, true);
  };
  const outside = (ev: MouseEvent) => {
    if (!menu.contains(ev.target as Node)) close();
  };
  setTimeout(() => document.addEventListener('mousedown', outside, true));
}
