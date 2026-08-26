use crate::components::{use_toast, Icon};
use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::components::A;
use leptos_router::hooks::use_navigate;

#[component]
pub fn RetrieveBookingPage() -> impl IntoView {
    let navigate = use_navigate();
    let toast = use_toast();
    let booking_ref = RwSignal::new(String::new());
    let contact = RwSignal::new(String::new());
    let error = RwSignal::new(Option::<&'static str>::None);
    let scan_notice = RwSignal::new(false);

    let find = {
        let navigate = navigate.clone();
        move |_: leptos::ev::MouseEvent| {
            let reference = booking_ref.get().trim().to_uppercase();
            let contact_value = contact.get();

            if reference.is_empty() {
                error.set(Some("Enter the booking reference from your confirmation."));
                toast.error("Reference missing", "We need the reference to find your reservation.");
                return;
            }
            if !reference.starts_with("HA-") || reference.len() < 6 {
                error.set(Some("References look like HA-250143 — check for a typo."));
                toast.error("That reference looks wrong", "References always start with HA- followed by six digits.");
                return;
            }
            if contact_value.trim().is_empty() {
                error.set(Some("Enter the phone number or email used to reserve."));
                toast.error("Contact missing", "We verify ownership with the contact details on the booking.");
                return;
            }

            error.set(None);
            toast.success("Reservation found", format!("Opening {reference}."));
            navigate(&format!("/reservation/{reference}"), Default::default());
        }
    };
    let find = StoredValue::new(find);

    let field = "w-full rounded-xl border border-slate-300 px-3.5 py-3 text-sm text-slate-800 transition-all duration-200 focus:border-blue-500 focus:outline-none focus:ring-4 focus:ring-blue-100";

    view! {
        <Title text="Retrieve your booking — Horn of Africa Hotel Portal" />

        <div class="mx-auto max-w-md px-4 py-10 pb-24 sm:pb-10">
            <div class="text-center">
                <div class="relative mx-auto mb-5 flex h-16 w-16 items-center justify-center">
                    <span class="absolute inset-0 animate-pulse-ring rounded-full bg-blue-300"></span>
                    <span class="relative flex h-16 w-16 animate-scale-in items-center justify-center rounded-2xl bg-blue-700 text-white shadow-xl shadow-blue-700/30">
                        <Icon name="search" class="h-7 w-7" />
                    </span>
                </div>
                <h1 class="animate-fade-up text-2xl font-extrabold tracking-tight text-slate-900" style="animation-delay: 80ms">
                    "Retrieve your booking"
                </h1>
                <p class="mx-auto mt-2 max-w-sm animate-fade-up text-sm leading-relaxed text-slate-500" style="animation-delay: 120ms">
                    "Enter the reference from your confirmation along with the phone number or email you used to reserve."
                </p>
            </div>

            <div class="mt-7 animate-fade-up rounded-2xl border border-slate-200 bg-white p-6 shadow-lg shadow-slate-900/5" style="animation-delay: 160ms">
                <Show when=move || error.get().is_some()>
                    <div class="mb-4 flex animate-fade-up items-start gap-2.5 rounded-xl bg-red-50 p-3.5 text-sm text-red-800 ring-1 ring-red-100">
                        <Icon name="alert" class="mt-0.5 h-4 w-4 shrink-0" />
                        <span>{move || error.get().unwrap_or("")}</span>
                    </div>
                </Show>

                <div class="flex flex-col gap-4">
                    <div>
                        <label class="mb-1.5 block text-xs font-bold uppercase tracking-wide text-slate-500">
                            "Booking reference"
                        </label>
                        <div class="relative">
                            <Icon name="ticket" class="pointer-events-none absolute left-3.5 top-1/2 h-4 w-4 -translate-y-1/2 text-slate-400" />
                            <input
                                type="text"
                                placeholder="HA-250143"
                                class=format!("{field} pl-10 font-mono uppercase tracking-wider")
                                prop:value=booking_ref
                                on:input:target=move |ev| { booking_ref.set(ev.target().value()); error.set(None); }
                            />
                        </div>
                    </div>

                    <div>
                        <label class="mb-1.5 block text-xs font-bold uppercase tracking-wide text-slate-500">
                            "Phone number or email"
                        </label>
                        <div class="relative">
                            <Icon name="phone" class="pointer-events-none absolute left-3.5 top-1/2 h-4 w-4 -translate-y-1/2 text-slate-400" />
                            <input
                                type="text"
                                placeholder="0912 345 678 or you@example.com"
                                class=format!("{field} pl-10")
                                prop:value=contact
                                on:input:target=move |ev| { contact.set(ev.target().value()); error.set(None); }
                            />
                        </div>
                    </div>

                    <button
                        on:click=move |ev| find.with_value(|f| f(ev))
                        class="sheen mt-1 flex w-full items-center justify-center gap-2 rounded-xl bg-blue-700 py-3.5 text-sm font-bold text-white shadow-lg shadow-blue-700/25 transition-all duration-200 hover:-translate-y-0.5 hover:bg-blue-800 hover:shadow-xl active:scale-[0.98]"
                    >
                        <Icon name="search" class="h-4 w-4" />
                        "Find my reservation"
                    </button>
                </div>

                <div class="my-5 flex items-center gap-3">
                    <span class="h-px flex-1 bg-slate-200"></span>
                    <span class="text-xs font-semibold uppercase tracking-wider text-slate-400">"or"</span>
                    <span class="h-px flex-1 bg-slate-200"></span>
                </div>

                <button
                    on:click=move |_| scan_notice.set(true)
                    class="flex w-full items-center gap-3.5 rounded-xl border border-blue-100 bg-blue-50/70 p-4 text-left transition-all duration-200 hover:-translate-y-0.5 hover:border-blue-200 hover:bg-blue-50 hover:shadow-md"
                >
                    <span class="flex h-10 w-10 shrink-0 items-center justify-center rounded-xl bg-blue-100 text-blue-700">
                        <Icon name="scan" class="h-5 w-5" />
                    </span>
                    <span class="min-w-0 flex-1">
                        <span class="block text-sm font-bold text-blue-700">"Scan your confirmation"</span>
                        <span class="block text-xs leading-relaxed text-slate-500">
                            "Point your camera at the reference on a printed or on-screen confirmation."
                        </span>
                    </span>
                    <Icon name="chevron-right" class="h-4 w-4 shrink-0 text-slate-400" />
                </button>

                <Show when=move || scan_notice.get()>
                    <div class="mt-3 flex animate-fade-up items-start gap-2.5 rounded-xl border border-amber-200 bg-amber-50 p-3.5 text-sm text-amber-900">
                        <Icon name="info" class="mt-0.5 h-4 w-4 shrink-0" />
                        <span>
                            "Camera scanning is not available in this preview build. Type the reference above instead — it is printed at the top of every confirmation."
                        </span>
                    </div>
                </Show>
            </div>

            <div class="mt-4 flex animate-fade-up items-start gap-2.5 rounded-2xl bg-emerald-50 p-4 text-sm text-emerald-900 ring-1 ring-emerald-100" style="animation-delay: 220ms">
                <Icon name="sparkles" class="mt-0.5 h-4 w-4 shrink-0" />
                <span>
                    <span class="font-bold">"Tip: "</span>
                    "Your reference is in the confirmation you downloaded, the WhatsApp message you shared, or any screenshot you took after booking."
                </span>
            </div>

            <div class="mt-5 flex animate-fade-up flex-col items-center gap-2 rounded-2xl border border-slate-200 bg-white px-6 py-7 text-center" style="animation-delay: 260ms">
                <span class="flex h-11 w-11 items-center justify-center rounded-2xl bg-slate-100 text-slate-600">
                    <Icon name="headset" class="h-5 w-5" />
                </span>
                <p class="text-sm font-bold text-slate-900">"Still can't find it?"</p>
                <p class="max-w-xs text-xs leading-relaxed text-slate-500">
                    "Send us your name and roughly when you were staying, and we will trace the reservation with the hotel."
                </p>
                <A
                    href="/contact"
                    attr:class="mt-2 flex items-center gap-2 rounded-xl border border-slate-300 px-5 py-2.5 text-sm font-bold text-slate-700 transition-all duration-200 hover:-translate-y-0.5 hover:border-blue-300 hover:text-blue-700 hover:shadow-md"
                >
                    <Icon name="headset" class="h-4 w-4" />
                    "Contact support"
                </A>
            </div>
        </div>
    }
}
