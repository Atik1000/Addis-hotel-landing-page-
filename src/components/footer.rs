use crate::components::{use_toast, Icon};
use leptos::prelude::*;
use leptos_router::components::A;

struct Column {
    heading: &'static str,
    links: &'static [(&'static str, &'static str)],
}

const COLUMNS: &[Column] = &[
    Column {
        heading: "Explore",
        links: &[
            ("Home", "/"),
            ("Browse Hotels", "/hotels"),
            ("How It Works", "/how-it-works"),
            ("About Us", "/about"),
        ],
    },
    Column {
        heading: "Your Booking",
        links: &[
            ("My Reservation", "/my-reservations"),
            ("Retrieve Booking", "/retrieve-booking"),
            ("Help Centre", "/faq"),
            ("Contact Support", "/contact"),
        ],
    },
    Column {
        heading: "Legal",
        links: &[
            ("Terms & Conditions", "/terms"),
            ("Privacy Policy", "/privacy"),
            ("Cancellation Policy", "/terms"),
            ("List Your Hotel", "/contact"),
        ],
    },
];

const DESTINATIONS: &[(&str, &str)] = &[
    ("Hotels in Addis Ababa", "/hotels?city=Addis+Ababa"),
    ("Hotels in Nairobi", "/hotels?city=Nairobi"),
    ("Hotels in Mogadishu", "/hotels?city=Mogadishu"),
    ("Hotels in Hargeisa", "/hotels?city=Hargeisa"),
    ("Hotels in Djibouti City", "/hotels?city=Djibouti+City"),
    ("Hotels in Dire Dawa", "/hotels?city=Dire+Dawa"),
];

#[component]
pub fn Footer() -> impl IntoView {
    let toast = use_toast();
    let email = RwSignal::new(String::new());

    let subscribe = move |_| {
        let value = email.get();
        if !value.contains('@') || !value.contains('.') {
            toast.error("Check your email", "Enter a valid email address so we can reach you.");
            return;
        }
        toast.success("You're subscribed", format!("Deals for the Horn of Africa will land in {value}."));
        email.set(String::new());
    };

    view! {
        <footer class="mt-16 border-t border-slate-200 bg-white">
            // ---- Newsletter ------------------------------------------------
            <div class="mx-auto max-w-6xl px-4 pt-12">
                <div class="reveal relative overflow-hidden rounded-3xl bg-gradient-to-br from-blue-700 via-blue-800 to-indigo-900 px-6 py-9 text-white shadow-xl shadow-blue-900/25 sm:px-10">
                    <span class="pointer-events-none absolute -right-16 -top-16 h-56 w-56 rounded-full bg-white/10 blur-2xl"></span>
                    <span class="pointer-events-none absolute -bottom-20 -left-10 h-56 w-56 rounded-full bg-cyan-400/15 blur-2xl"></span>

                    <div class="relative flex flex-col items-start justify-between gap-6 lg:flex-row lg:items-center">
                        <div class="max-w-md">
                            <span class="inline-flex items-center gap-1.5 rounded-full bg-white/15 px-3 py-1 text-[11px] font-bold uppercase tracking-wider ring-1 ring-white/20">
                                <Icon name="gift" class="h-3.5 w-3.5" />
                                "Members save more"
                            </span>
                            <h2 class="mt-3 text-2xl font-extrabold tracking-tight">"Get regional hotel deals first"</h2>
                            <p class="mt-1.5 text-sm leading-relaxed text-blue-100">
                                "One email a month with the best verified rates across Ethiopia, Kenya, Somalia, Somaliland and Djibouti. No spam, unsubscribe any time."
                            </p>
                        </div>

                        <div class="w-full max-w-md">
                            <div class="flex flex-col gap-2.5 sm:flex-row">
                                <div class="relative flex-1">
                                    <Icon name="mail" class="pointer-events-none absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-slate-400" />
                                    <input
                                        type="email"
                                        placeholder="you@example.com"
                                        class="w-full rounded-xl border border-white/20 bg-white py-3 pl-9 pr-3 text-sm text-slate-800 placeholder:text-slate-400 focus:outline-none focus:ring-4 focus:ring-white/25"
                                        prop:value=email
                                        on:input:target=move |ev| email.set(ev.target().value())
                                    />
                                </div>
                                <button
                                    on:click=subscribe
                                    class="sheen shrink-0 rounded-xl bg-white px-6 py-3 text-sm font-bold text-blue-800 shadow-lg transition-all duration-200 hover:-translate-y-0.5 hover:shadow-xl active:scale-[0.98]"
                                >
                                    "Subscribe"
                                </button>
                            </div>
                            <p class="mt-2 flex items-center gap-1.5 text-xs text-blue-200">
                                <Icon name="shield-check" class="h-3.5 w-3.5" />
                                "We never share your address with hotels or third parties."
                            </p>
                        </div>
                    </div>
                </div>
            </div>

            // ---- Link columns ----------------------------------------------
            <div class="mx-auto max-w-6xl px-4 py-12">
                <div class="grid gap-9 sm:grid-cols-2 lg:grid-cols-5">
                    <div class="lg:col-span-2">
                        <A href="/" attr:class="group flex items-center gap-2.5">
                            <span class="flex h-9 w-9 items-center justify-center rounded-xl bg-gradient-to-br from-blue-600 to-indigo-700 text-white shadow-md transition-transform duration-300 group-hover:scale-105">
                                <Icon name="building" class="h-5 w-5" />
                            </span>
                            <span class="leading-tight">
                                <span class="block text-base font-extrabold text-slate-900">"Horn of Africa"</span>
                                <span class="block text-[10px] font-semibold tracking-[0.16em] text-blue-700">"HOTEL PORTAL"</span>
                            </span>
                        </A>
                        <p class="mt-4 max-w-sm text-sm leading-relaxed text-slate-500">
                            "Discover and reserve verified hotels across the Horn of Africa. Pay at the hotel, no booking fees, no account required — the rate you see is the rate the hotel charges."
                        </p>

                        <div class="mt-5 flex items-center gap-2">
                            <SocialIcon icon="facebook" label="Facebook" />
                            <SocialIcon icon="instagram" label="Instagram" />
                            <SocialIcon icon="twitter" label="X" />
                            <SocialIcon icon="message" label="WhatsApp" />
                        </div>

                        <div class="mt-5 flex flex-col gap-2 text-sm text-slate-600">
                            <a href="tel:+251111234567" class="flex items-center gap-2 transition-colors hover:text-blue-700">
                                <Icon name="phone" class="h-3.5 w-3.5 text-blue-700" />
                                "+251 11 123 4567"
                            </a>
                            <a href="mailto:support@hornofafrica-hotels.com" class="flex items-center gap-2 transition-colors hover:text-blue-700">
                                <Icon name="mail" class="h-3.5 w-3.5 text-blue-700" />
                                "support@hornofafrica-hotels.com"
                            </a>
                        </div>
                    </div>

                    {COLUMNS.iter().map(|col| view! {
                        <div>
                            <h3 class="text-xs font-bold uppercase tracking-wider text-slate-400">{col.heading}</h3>
                            <ul class="mt-3.5 flex flex-col gap-2.5 text-sm">
                                {col.links.iter().map(|(label, href)| view! {
                                    <li>
                                        <A
                                            href=*href
                                            attr:class="inline-flex items-center gap-1.5 text-slate-600 transition-all duration-200 hover:translate-x-0.5 hover:text-blue-700"
                                        >
                                            <Icon name="chevron-right" class="h-3 w-3 text-slate-300" />
                                            {*label}
                                        </A>
                                    </li>
                                }).collect_view()}
                            </ul>
                        </div>
                    }).collect_view()}
                </div>

                // ---- Popular destinations -----------------------------------
                <div class="mt-10 border-t border-slate-100 pt-7">
                    <h3 class="text-xs font-bold uppercase tracking-wider text-slate-400">"Popular destinations"</h3>
                    <div class="mt-3 flex flex-wrap gap-2">
                        {DESTINATIONS.iter().map(|(label, href)| view! {
                            <a
                                href=*href
                                class="rounded-full border border-slate-200 px-3 py-1.5 text-xs font-medium text-slate-600 transition-all duration-200 hover:-translate-y-0.5 hover:border-blue-300 hover:bg-blue-50 hover:text-blue-700"
                            >
                                {*label}
                            </a>
                        }).collect_view()}
                    </div>
                </div>
            </div>

            // ---- Bottom bar --------------------------------------------------
            <div class="border-t border-slate-200 bg-slate-50">
                <div class="mx-auto flex max-w-6xl flex-col items-center justify-between gap-3 px-4 py-5 text-xs text-slate-500 sm:flex-row">
                    <span>"© 2026 Horn of Africa Hotel Portal. All rights reserved."</span>
                    <div class="flex flex-wrap items-center justify-center gap-4">
                        <span class="flex items-center gap-1.5">
                            <Icon name="shield-check" class="h-3.5 w-3.5 text-emerald-600" />
                            "Verified listings"
                        </span>
                        <span class="flex items-center gap-1.5">
                            <Icon name="globe" class="h-3.5 w-3.5" />
                            "English · ETB"
                        </span>
                        <A href="/terms" attr:class="transition-colors hover:text-blue-700 hover:underline">"Terms"</A>
                        <A href="/privacy" attr:class="transition-colors hover:text-blue-700 hover:underline">"Privacy"</A>
                    </div>
                </div>
            </div>
        </footer>
    }
}

#[component]
fn SocialIcon(icon: &'static str, label: &'static str) -> impl IntoView {
    view! {
        <a
            href="#"
            aria-label=label
            class="flex h-9 w-9 items-center justify-center rounded-xl border border-slate-200 bg-white text-slate-500 transition-all duration-200 hover:-translate-y-1 hover:border-blue-300 hover:bg-blue-50 hover:text-blue-700 hover:shadow-md"
        >
            <Icon name=icon class="h-4 w-4" />
        </a>
    }
}
