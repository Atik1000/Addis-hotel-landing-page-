//! Retrieving a booking, backed by the two public OTP endpoints.
//!
//! There are no accounts, so a booking is proven by possession of the contact
//! detail on it: `request-otp/` sends a six-digit code to the email or phone
//! recorded against the reference, and `retrieve/` exchanges that code for the
//! reservation. The result is cached in [`crate::store`] so this browser can
//! show it again without a second code.

use crate::api::{request_booking_otp, retrieve_booking};
use crate::components::{use_toast, Icon};
use crate::store;
use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::components::A;
use leptos_router::hooks::use_navigate;

#[derive(Clone, Copy, PartialEq)]
enum Step {
    Identify,
    Verify,
}

#[component]
pub fn RetrieveBookingPage() -> impl IntoView {
    let toast = use_toast();
    let navigate = use_navigate();
    let nav = StoredValue::new(navigate);

    let step = RwSignal::new(Step::Identify);
    let reference = RwSignal::new(String::new());
    let email = RwSignal::new(String::new());
    let phone = RwSignal::new(String::new());
    let code = RwSignal::new(String::new());
    let error = RwSignal::new(Option::<String>::None);
    let busy = RwSignal::new(false);

    let field = "w-full rounded-xl border border-slate-300 px-3.5 py-2.5 text-sm text-slate-800 transition-all duration-200 focus:border-blue-500 focus:outline-none focus:ring-4 focus:ring-blue-100";

    let send_code = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        if busy.get() {
            return;
        }
        let r = reference.get().trim().to_uppercase();
        if r.len() < 5 {
            error.set(Some("Enter the booking reference from your confirmation.".into()));
            return;
        }
        if email.get().trim().is_empty() && phone.get().trim().is_empty() {
            error.set(Some(
                "Enter the email address or phone number on the booking.".into(),
            ));
            return;
        }
        error.set(None);
        busy.set(true);
        reference.set(r.clone());
        let (e, p) = (email.get(), phone.get());

        leptos::task::spawn_local(async move {
            match request_booking_otp(r, e, p).await {
                Ok(_) => {
                    busy.set(false);
                    step.set(Step::Verify);
                    toast.success("Code sent", "Check your email or phone for a six-digit code.");
                }
                Err(err) => {
                    error.set(Some(err.to_string()));
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
        if code.get().trim().len() != 6 {
            error.set(Some("The code is six digits.".into()));
            return;
        }
        error.set(None);
        busy.set(true);
        let (r, e, p, c) = (reference.get(), email.get(), phone.get(), code.get());

        leptos::task::spawn_local(async move {
            match retrieve_booking(r, e, p, c).await {
                Ok(res) => {
                    // Remembered so "My reservations" can list it without
                    // another code.
                    store::remember(&res);
                    let target = format!("/reservation/{}", res.reference());
                    toast.success("Booking found", "Here are your reservation details.");
                    nav.with_value(|n| n(&target, Default::default()));
                }
                Err(err) => {
                    error.set(Some(err.to_string()));
                    busy.set(false);
                }
            }
        });
    };

    let resend = move |_| {
        if busy.get() {
            return;
        }
        busy.set(true);
        error.set(None);
        let (r, e, p) = (reference.get(), email.get(), phone.get());
        leptos::task::spawn_local(async move {
            match request_booking_otp(r, e, p).await {
                Ok(_) => toast.success("Code resent", "A new code is on its way."),
                Err(err) => error.set(Some(err.to_string())),
            }
            busy.set(false);
        });
    };

    view! {
        <Title text="Retrieve your booking — Horn of Africa Hotel Portal" />

        <div class="mx-auto max-w-lg px-4 py-12">
            <div class="rounded-2xl border border-slate-200 bg-white p-6 shadow-sm">
                <div class="flex flex-col items-center text-center">
                    <span class="mb-3 flex h-14 w-14 items-center justify-center rounded-2xl bg-blue-50 text-blue-700">
                        <Icon name="key" class="h-6 w-6" />
                    </span>
                    <h1 class="text-xl font-bold text-slate-900">"Find your booking"</h1>
                    <p class="mt-1 text-sm text-slate-500">
                        "No account needed. We'll send a one-time code to the contact details on the reservation."
                    </p>
                </div>

                <Show when=move || error.get().is_some()>
                    <div class="mt-5 flex items-start gap-2 rounded-xl bg-red-50 px-3 py-2.5 text-sm text-red-700">
                        <Icon name="alert" class="mt-0.5 h-4 w-4 shrink-0" />
                        {move || error.get().unwrap_or_default()}
                    </div>
                </Show>

                // ---- Step 1: identify the booking --------------------
                <Show when=move || step.get() == Step::Identify>
                    <form on:submit=send_code class="mt-5 flex flex-col gap-4">
                        <div>
                            <label class="mb-1.5 block text-sm font-semibold text-slate-700">"Booking reference"</label>
                            <input
                                type="text"
                                placeholder="HA-260828513"
                                class=format!("{field} uppercase tracking-wider")
                                prop:value=reference
                                on:input:target=move |ev| { reference.set(ev.target().value()); error.set(None); }
                            />
                        </div>

                        <div class="flex items-center gap-3">
                            <span class="h-px flex-1 bg-slate-200"></span>
                            <span class="text-xs font-semibold uppercase tracking-wide text-slate-400">"and one of"</span>
                            <span class="h-px flex-1 bg-slate-200"></span>
                        </div>

                        <div>
                            <label class="mb-1.5 block text-sm font-semibold text-slate-700">"Email on the booking"</label>
                            <input type="email" placeholder="you@example.com" class=field
                                prop:value=email
                                on:input:target=move |ev| { email.set(ev.target().value()); error.set(None); } />
                        </div>
                        <div>
                            <label class="mb-1.5 block text-sm font-semibold text-slate-700">"Phone on the booking"</label>
                            <input type="tel" placeholder="+251 …" class=field
                                prop:value=phone
                                on:input:target=move |ev| { phone.set(ev.target().value()); error.set(None); } />
                        </div>

                        <button
                            type="submit"
                            disabled=move || busy.get()
                            class="sheen mt-1 flex items-center justify-center gap-2 rounded-xl bg-blue-700 py-3 text-sm font-bold text-white shadow-md shadow-blue-700/20 transition-all duration-200 hover:bg-blue-800 active:scale-[0.98] disabled:opacity-60"
                        >
                            <Icon name="send" class="h-4 w-4" />
                            {move || if busy.get() { "Sending…" } else { "Send my code" }}
                        </button>
                    </form>
                </Show>

                // ---- Step 2: verify the code -------------------------
                <Show when=move || step.get() == Step::Verify>
                    <form on:submit=verify class="mt-5 flex flex-col gap-4">
                        <p class="text-sm text-slate-600">
                            "We sent a six-digit code for "
                            <span class="font-bold text-slate-900">{move || reference.get()}</span>
                            ". Enter it below."
                        </p>
                        <div>
                            <label class="mb-1.5 block text-sm font-semibold text-slate-700">"Verification code"</label>
                            <input
                                type="text"
                                inputmode="numeric"
                                maxlength="6"
                                placeholder="123456"
                                class=format!("{field} text-center text-lg font-bold tracking-[0.4em] tabular-nums")
                                prop:value=code
                                on:input:target=move |ev| { code.set(ev.target().value()); error.set(None); }
                            />
                        </div>

                        <button
                            type="submit"
                            disabled=move || busy.get()
                            class="sheen flex items-center justify-center gap-2 rounded-xl bg-blue-700 py-3 text-sm font-bold text-white shadow-md shadow-blue-700/20 transition-all duration-200 hover:bg-blue-800 active:scale-[0.98] disabled:opacity-60"
                        >
                            <Icon name="check" class="h-4 w-4" />
                            {move || if busy.get() { "Checking…" } else { "View my booking" }}
                        </button>

                        <div class="flex items-center justify-between text-xs">
                            <button
                                type="button"
                                class="font-semibold text-slate-500 hover:text-slate-800"
                                on:click=move |_| { step.set(Step::Identify); error.set(None); }
                            >
                                "Change details"
                            </button>
                            <button
                                type="button"
                                class="font-semibold text-blue-700 hover:underline disabled:opacity-50"
                                disabled=move || busy.get()
                                on:click=resend
                            >
                                "Resend code"
                            </button>
                        </div>
                    </form>
                </Show>

                <div class="mt-6 border-t border-slate-100 pt-5 text-center text-sm text-slate-500">
                    "Booked in this browser before? "
                    <A href="/my-reservations" attr:class="font-semibold text-blue-700 hover:underline">
                        "See your saved bookings"
                    </A>
                </div>
            </div>
        </div>
    }
}
