//! Lightweight toast notifications.
//!
//! [`provide_toasts`] is called once from `App`; any descendant can then call
//! [`use_toast`] and push a message. [`ToastHost`] renders the stack.

use crate::components::Icon;
use leptos::prelude::*;
use std::time::Duration;

/// How long a toast stays on screen before it removes itself.
const TOAST_TTL: Duration = Duration::from_secs(5);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToastKind {
    Success,
    Info,
    Warning,
    Error,
}

impl ToastKind {
    fn icon(self) -> &'static str {
        match self {
            ToastKind::Success => "check-circle",
            ToastKind::Info => "info",
            ToastKind::Warning => "alert",
            ToastKind::Error => "x-circle",
        }
    }

    fn accent(self) -> &'static str {
        match self {
            ToastKind::Success => "bg-emerald-50 text-emerald-700 ring-emerald-200",
            ToastKind::Info => "bg-blue-50 text-blue-700 ring-blue-200",
            ToastKind::Warning => "bg-amber-50 text-amber-700 ring-amber-200",
            ToastKind::Error => "bg-red-50 text-red-700 ring-red-200",
        }
    }

    fn bar(self) -> &'static str {
        match self {
            ToastKind::Success => "bg-emerald-500",
            ToastKind::Info => "bg-blue-600",
            ToastKind::Warning => "bg-amber-500",
            ToastKind::Error => "bg-red-500",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Toast {
    pub id: u32,
    pub kind: ToastKind,
    pub title: String,
    pub body: String,
}

#[derive(Copy, Clone)]
pub struct ToastCtx {
    items: RwSignal<Vec<Toast>>,
    next_id: RwSignal<u32>,
}

impl ToastCtx {
    pub fn push(&self, kind: ToastKind, title: impl Into<String>, body: impl Into<String>) {
        let id = self.next_id.get_untracked();
        self.next_id.set(id + 1);
        self.items.update(|list| {
            list.push(Toast { id, kind, title: title.into(), body: body.into() });
            // Never stack more than three at once.
            if list.len() > 3 {
                list.remove(0);
            }
        });

        // Auto-dismiss; the close button can still retire it sooner. Only ever
        // reached from a client-side event handler, so the timer is safe here.
        let items = self.items;
        set_timeout(
            move || items.update(|list| list.retain(|t| t.id != id)),
            TOAST_TTL,
        );
    }

    pub fn success(&self, title: impl Into<String>, body: impl Into<String>) {
        self.push(ToastKind::Success, title, body);
    }

    pub fn info(&self, title: impl Into<String>, body: impl Into<String>) {
        self.push(ToastKind::Info, title, body);
    }

    pub fn error(&self, title: impl Into<String>, body: impl Into<String>) {
        self.push(ToastKind::Error, title, body);
    }

    pub fn dismiss(&self, id: u32) {
        self.items.update(|list| list.retain(|t| t.id != id));
    }
}

pub fn provide_toasts() {
    provide_context(ToastCtx {
        items: RwSignal::new(Vec::new()),
        next_id: RwSignal::new(1),
    });
}

/// Panics only if `provide_toasts` was never called, which would be a wiring bug.
pub fn use_toast() -> ToastCtx {
    expect_context::<ToastCtx>()
}

#[component]
pub fn ToastHost() -> impl IntoView {
    let ctx = use_toast();

    view! {
        <div class="pointer-events-none fixed bottom-4 right-4 z-[60] flex w-[min(22rem,calc(100vw-2rem))] flex-col gap-2.5">
            <For
                each=move || ctx.items.get()
                key=|t| t.id
                let:toast
            >
                {
                    let id = toast.id;
                    let kind = toast.kind;
                    view! {
                        <div class=format!(
                            "pointer-events-auto relative animate-toast-in overflow-hidden rounded-xl bg-white p-3.5 shadow-xl shadow-slate-900/10 ring-1 {}",
                            kind.accent()
                        )>
                            <div class="flex items-start gap-3">
                                <Icon name=kind.icon() class="mt-0.5 h-4.5 w-4.5 shrink-0" />
                                <div class="min-w-0 flex-1">
                                    <p class="text-sm font-bold">{toast.title.clone()}</p>
                                    <p class="mt-0.5 text-xs leading-relaxed opacity-80">{toast.body.clone()}</p>
                                </div>
                                <button
                                    aria-label="Dismiss"
                                    class="-m-1 shrink-0 rounded p-1 opacity-50 transition-opacity hover:opacity-100"
                                    on:click=move |_| ctx.dismiss(id)
                                >
                                    <Icon name="x" class="h-3.5 w-3.5" />
                                </button>
                            </div>
                            <span class=format!("absolute inset-x-0 bottom-0 h-0.5 {}", kind.bar())></span>
                        </div>
                    }
                }
            </For>
        </div>
    }
}
