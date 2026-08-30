use crate::components::{AccordionItem, Icon, SectionHeading, TrustBar};
use crate::data::{FAQS, HOW_IT_WORKS};
use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::components::A;

const DETAIL: &[(&str, &str, &[&str])] = &[
    (
        "Search",
        "Start with a city and your dates.",
        &[
            "Type a city, an area or a hotel name — the destination box suggests as you go.",
            "Set your check-in and check-out dates and how many guests and rooms you need.",
            "Results show every verified property with live rates. Nothing is boosted for payment.",
            "Narrow things down with price, star class, guest rating and amenity filters.",
        ],
    ),
    (
        "Choose your room",
        "Compare room types like you would in person.",
        &[
            "Each room shows real photographs, the exact bed configuration, size in square metres and how many it sleeps.",
            "Rates are per room per night and always include what the hotel includes — breakfast, Wi-Fi, pool access.",
            "Look for the free-cancellation badge if your plans might move.",
            "Rooms running low show how many are left at that rate.",
        ],
    ),
    (
        "Reserve in seconds",
        "A name and a contact number is all it takes.",
        &[
            "No account to create, no password, no verification email.",
            "We never ask for card details — there is nowhere on the site to enter them.",
            "Add special requests (high floor, cot, late arrival) and the hotel sees them with the booking.",
            "You get a booking reference immediately, and the hotel gets the reservation at the same moment.",
        ],
    ),
    (
        "Pay at the hotel",
        "Settle up at the front desk, at the quoted rate.",
        &[
            "Show your booking reference and a passport or national ID at check-in.",
            "Pay in cash or by card — mobile money at properties that accept it.",
            "The total on your confirmation is the total you pay. No booking fee is ever added.",
            "Need to cancel? Free up to 24 hours before check-in on most rates.",
        ],
    ),
];

#[component]
pub fn HowItWorksPage() -> impl IntoView {
    view! {
        <Title text="How it works — Horn of Africa Hotel Portal" />

        <section class="relative overflow-hidden border-b border-slate-200 bg-gradient-to-br from-blue-50 via-white to-indigo-50">
            <span class="pointer-events-none absolute -right-24 -top-24 h-72 w-72 rounded-full bg-blue-200/40 blur-3xl"></span>
            <span class="pointer-events-none absolute -bottom-24 -left-16 h-72 w-72 rounded-full bg-cyan-200/40 blur-3xl"></span>

            <div class="relative mx-auto max-w-3xl px-4 py-16 text-center">
                <span class="inline-flex animate-fade-up items-center gap-2 rounded-full bg-white px-3.5 py-1.5 text-xs font-bold uppercase tracking-wider text-blue-700 shadow-sm ring-1 ring-blue-100">
                    <Icon name="sparkles" class="h-3.5 w-3.5" />
                    "Under a minute, start to finish"
                </span>
                <h1 class="mt-5 animate-fade-up text-4xl font-bold tracking-tight text-ink sm:text-5xl" style="animation-delay: 80ms">
                    "How booking here works"
                </h1>
                <p class="mx-auto mt-4 max-w-xl animate-fade-up text-base leading-relaxed text-slate-600" style="animation-delay: 130ms">
                    "Four steps, no account, no card. Here is exactly what happens between searching and walking into your room."
                </p>
            </div>
        </section>

        <section class="mx-auto max-w-6xl px-4 py-14">
            <div class="relative">
                <span class="pointer-events-none absolute left-[12%] right-[12%] top-9 hidden h-0.5 bg-gradient-to-r from-blue-200 via-blue-300 to-blue-200 lg:block"></span>
                <div class="grid gap-6 sm:grid-cols-2 lg:grid-cols-4">
                    {HOW_IT_WORKS.iter().enumerate().map(|(i, step)| {
                        let delay = format!("animation-delay: {}ms", i * 90);
                        view! {
                            <div class="reveal group relative flex flex-col items-center text-center" style=delay>
                                <span class="relative z-10 flex h-[4.5rem] w-[4.5rem] items-center justify-center rounded-2xl border border-blue-100 bg-white text-blue-700 shadow-md transition-all duration-300 group-hover:-translate-y-1.5 group-hover:bg-blue-700 group-hover:text-white group-hover:shadow-xl">
                                    <Icon name=step.icon class="h-7 w-7" />
                                    <span class="absolute -right-2 -top-2 flex h-7 w-7 items-center justify-center rounded-full bg-blue-700 text-[11px] font-bold text-white ring-4 ring-slate-50 group-hover:bg-slate-900">
                                        {step.number}
                                    </span>
                                </span>
                                <h2 class="mt-4 text-base font-bold text-ink">{step.title}</h2>
                                <p class="mt-1.5 text-sm leading-relaxed text-slate-500">{step.body}</p>
                            </div>
                        }
                    }).collect_view()}
                </div>
            </div>
        </section>

        <section class="border-y border-slate-200 bg-white py-14">
            <div class="mx-auto max-w-4xl px-4">
                <div class="flex flex-col gap-10">
                    {DETAIL.iter().enumerate().map(|(i, (title, lede, points))| {
                        let delay = format!("animation-delay: {}ms", (i % 2) * 80);
                        view! {
                            <div class="reveal grid gap-5 sm:grid-cols-[auto_minmax(0,1fr)]" style=delay>
                                <span class="flex h-12 w-12 shrink-0 items-center justify-center rounded-2xl bg-blue-700 text-lg font-bold text-white shadow-lg shadow-blue-700/25">
                                    {i + 1}
                                </span>
                                <div>
                                    <h2 class="text-xl font-bold tracking-tight text-ink">{*title}</h2>
                                    <p class="mt-1 text-sm font-medium text-blue-700">{*lede}</p>
                                    <ul class="mt-3 flex flex-col gap-2">
                                        {points.iter().map(|p| view! {
                                            <li class="flex items-start gap-2.5 text-sm leading-relaxed text-slate-600">
                                                <Icon name="check-circle" class="mt-0.5 h-4 w-4 shrink-0 text-emerald-600" />
                                                {*p}
                                            </li>
                                        }).collect_view()}
                                    </ul>
                                </div>
                            </div>
                        }
                    }).collect_view()}
                </div>
            </div>
        </section>

        <section class="mx-auto max-w-6xl px-4 py-14">
            <SectionHeading
                center=true
                eyebrow="The promise"
                title="What you get every single time"
            />
            <div class="mt-8">
                <TrustBar />
            </div>
        </section>

        <section class="border-t border-slate-200 bg-white py-14">
            <div class="mx-auto max-w-3xl px-4">
                <SectionHeading center=true eyebrow="Questions" title="Common questions about the flow" />
                <div class="mt-8 flex flex-col gap-3">
                    {FAQS.iter().filter(|f| f.category == "Booking" || f.category == "Payment").enumerate().map(|(i, f)| {
                        let delay = format!("animation-delay: {}ms", i * 50);
                        view! {
                            <div class="reveal" style=delay>
                                <AccordionItem question=f.question answer=f.answer start_open=(i == 0) />
                            </div>
                        }
                    }).collect_view()}
                </div>
            </div>
        </section>

        <section class="mx-auto max-w-4xl px-4 py-14">
            <div class="reveal flex flex-col items-center gap-3 rounded-3xl bg-gradient-to-br from-blue-700 to-indigo-800 px-8 py-12 text-center text-white shadow-xl shadow-blue-900/25">
                <Icon name="search" class="h-9 w-9 animate-float" />
                <h2 class="text-2xl font-bold tracking-tight">"Ready to try it?"</h2>
                <p class="max-w-md text-sm leading-relaxed text-blue-100">
                    "Pick a city, choose a room and reserve. You will have a booking reference before you finish reading this page."
                </p>
                <A
                    href="/hotels"
                    attr:class="sheen mt-3 flex items-center gap-2 rounded-xl bg-white px-7 py-3.5 text-sm font-bold text-ink shadow-xl transition-all duration-200 hover:-translate-y-0.5 hover:shadow-2xl active:scale-[0.98]"
                >
                    "Browse hotels"
                    <Icon name="arrow-right" class="h-4 w-4" />
                </A>
            </div>
        </section>
    }
}
