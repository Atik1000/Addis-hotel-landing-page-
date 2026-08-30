//! `/sign-in` — passwordless guest sign-in.
//!
//! `POST /accounts/auth/otp/request/` sends a six-digit code to an email
//! address or phone number, and `POST /accounts/auth/otp/verify/` exchanges it
//! for a session. No password is ever involved, which suits a portal where
//! booking itself needs no account: you only sign in when you want to leave a
//! review or read back the ones you have left.

use crate::api::{request_guest_otp, verify_guest_otp};
use crate::components::{Icon, use_toast};
use crate::session;
use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::hooks::{use_navigate, use_query_map};

#[derive(Clone, Copy, PartialEq)]
enum Step {
    Identify,
    Verify,
}

/// `email` when the target looks like an address, `phone` otherwise — the API
/// rejects a mismatch rather than working it out itself.
fn channel_for(target: &str) -> &'static str {
    if target.contains('@') {
        "email"
    } else {
        "phone"
    }
}

#[component]
pub fn SignInPage() -> impl IntoView {
    let toast = use_toast();
    let navigate = use_navigate();
    let nav = StoredValue::new(navigate);
    let query = use_query_map();
    let guest = session::use_guest();

    // Where to land after signing in; defaults to the guest's own reviews.
    let next = move || {
        query
            .get()
            .get("next")
            .filter(|n| n.starts_with('/'))
            .unwrap_or_else(|| "/my-reviews".to_string())
    };

    let step = RwSignal::new(Step::Identify);
    let target = RwSignal::new(String::new());
    let code = RwSignal::new(String::new());
    let error = RwSignal::new(Option::<String>::None);
    let busy = RwSignal::new(false);

    let field = "w-full rounded-xl border border-slate-300 px-3.5 py-2.5 text-sm text-slate-800 transition-all duration-200 focus:border-blue-500 focus:outline-none focus:ring-4 focus:ring-blue-100";

    let send_code = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        if busy.get() {
            return;
        }
        let t = target.get().trim().to_string();
        if t.len() < 5 {
            error.set(Some("Enter your email address or phone number.".into()));
            return;
        }
        error.set(None);
        busy.set(true);
        let channel = channel_for(&t).to_string();

        leptos::task::spawn_local(async move {
            match request_guest_otp(t, channel).await {
                Ok(()) => {
                    busy.set(false);
                    step.set(Step::Verify);
                    toast.success("Code sent", "It is valid for a few minutes.");
                }
                Err(e) => {
                    error.set(Some(e.to_string()));
                    busy.set(false);
                }
            }
        });
    };

    let verify = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        if busy.get() {
            return;
        }
        let c = code.get().trim().to_string();
        if c.len() != 6 {
            error.set(Some("The code is six digits.".into()));
            return;
        }
        error.set(None);
        busy.set(true);
        let t = target.get();

        leptos::task::spawn_local(async move {
            match verify_guest_otp(t, c).await {
                Ok(s) => {
                    let who = s.short_name();
                    session::sign_in(s);
                    busy.set(false);
                    toast.success("Signed in", &format!("Welcome back, {who}."));
                    nav.with_value(|n| n(&next(), Default::default()));
                }
                Err(e) => {
                    error.set(Some(e.to_string()));
                    busy.set(false);
                }
            }
        });
    };

    view! {
        <Title text="Sign in — Horn of Africa Hotel Portal" />

        <div class="mx-auto max-w-md px-4 py-12">
            // Already signed in: offer the way onward rather than a dead form.
            <Show when=move || guest.get().is_some()>
                <div class="rounded-2xl border border-emerald-200 bg-emerald-50 p-5 text-center">
                    <span class="mx-auto mb-3 flex h-12 w-12 items-center justify-center rounded-full bg-emerald-100 text-emerald-700">
                        <Icon name="check-circle" class="h-6 w-6" />
                    </span>
                    <p class="text-sm font-bold text-emerald-900">
                        {move || format!(
                            "Signed in as {}",
                            guest.get().map(|g| g.short_name()).unwrap_or_default()
                        )}
                    </p>
                    <div class="mt-4 flex flex-col gap-2">
                        <a
                            href="/my-reviews"
                            class="rounded-xl bg-blue-700 py-2.5 text-sm font-bold text-white transition-colors hover:bg-blue-800"
                        >
                            "My reviews"
                        </a>
                        <button
                            on:click=move |_| {
                                session::sign_out();
                                toast.info("Signed out", "You can sign back in any time.");
                            }
                            class="rounded-xl border border-slate-300 py-2.5 text-sm font-bold text-slate-700 transition-colors hover:border-slate-400"
                        >
                            "Sign out"
                        </button>
                    </div>
                </div>
            </Show>

            <Show when=move || guest.get().is_none()>
                <div class="text-center">
                    <span class="mx-auto mb-4 flex h-12 w-12 items-center justify-center rounded-2xl bg-blue-50 text-blue-700">
                        <Icon name="key" class="h-6 w-6" />
                    </span>
                    <h1 class="text-2xl font-bold tracking-tight text-ink">"Sign in"</h1>
                    <p class="mx-auto mt-2 max-w-sm text-sm leading-relaxed text-slate-500">
                        "No password. We send a six-digit code to the email or phone you booked with."
                    </p>
                </div>

                {move || error.get().map(|e| view! {
                    <p class="mt-5 flex items-start gap-2 rounded-xl border border-red-200 bg-red-50 px-4 py-3 text-sm text-red-700">
                        <Icon name="alert" class="mt-0.5 h-4 w-4 shrink-0" />
                        {e}
                    </p>
                })}

                <Show when=move || step.get() == Step::Identify>
                    <form on:submit=send_code class="mt-6 flex flex-col gap-3">
                        <label class="text-sm font-semibold text-slate-700">
                            "Email or phone"
                            <input
                                class=format!("mt-1.5 {field}")
                                placeholder="you@example.com or +251…"
                                autocomplete="email"
                                prop:value=move || target.get()
                                on:input=move |e| target.set(event_target_value(&e))
                            />
                        </label>
                        <button
                            type="submit"
                            disabled=move || busy.get()
                            class="sheen mt-1 flex items-center justify-center gap-2 rounded-xl bg-blue-700 py-3 text-sm font-bold text-white shadow-md shadow-blue-700/25 transition-all duration-200 hover:bg-blue-800 disabled:cursor-not-allowed disabled:opacity-60"
                        >
                            {move || if busy.get() { "Sending…" } else { "Send me a code" }}
                            <Show when=move || !busy.get()>
                                <Icon name="arrow-right" class="h-4 w-4" />
                            </Show>
                        </button>
                    </form>
                </Show>

                <Show when=move || step.get() == Step::Verify>
                    <form on:submit=verify class="mt-6 flex flex-col gap-3">
                        <p class="rounded-xl border border-slate-200 bg-slate-50 px-4 py-3 text-sm text-slate-600">
                            "Code sent to "
                            <span class="font-bold text-ink">{move || target.get()}</span>
                        </p>
                        <label class="text-sm font-semibold text-slate-700">
                            "Six-digit code"
                            <input
                                class=format!("mt-1.5 text-center text-lg tracking-[0.4em] {field}")
                                placeholder="000000"
                                inputmode="numeric"
                                maxlength="6"
                                autocomplete="one-time-code"
                                prop:value=move || code.get()
                                on:input=move |e| code.set(event_target_value(&e))
                            />
                        </label>
                        <button
                            type="submit"
                            disabled=move || busy.get()
                            class="sheen mt-1 rounded-xl bg-blue-700 py-3 text-sm font-bold text-white shadow-md shadow-blue-700/25 transition-all duration-200 hover:bg-blue-800 disabled:cursor-not-allowed disabled:opacity-60"
                        >
                            {move || if busy.get() { "Checking…" } else { "Sign in" }}
                        </button>
                        <button
                            type="button"
                            on:click=move |_| { step.set(Step::Identify); code.set(String::new()); error.set(None); }
                            class="text-xs font-semibold text-slate-500 transition-colors hover:text-blue-700"
                        >
                            "Use a different email or phone"
                        </button>
                    </form>
                </Show>
            </Show>
        </div>
    }
}
