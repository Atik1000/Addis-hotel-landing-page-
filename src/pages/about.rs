use crate::components::{Icon, SectionHeading, TrustBar};
use crate::data::{HOW_IT_WORKS, STATS};
use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::components::A;

struct Value {
    icon: &'static str,
    title: &'static str,
    body: &'static str,
}

const VALUES: &[Value] = &[
    Value { icon: "shield-check", title: "Verified, not scraped", body: "Every listing is visited or video-verified by our team before it goes live. We do not import feeds from other portals and pass them off as first-hand." },
    Value { icon: "wallet", title: "The hotel's price, unchanged", body: "We take no commission from the guest and add no service fee. If a hotel quotes ETB 2,800 at the front desk, that is what you see here." },
    Value { icon: "users", title: "Built for how the region books", body: "Most travellers here do not want to hand card details to a website. Reserve with a name and a number, settle at the desk — the way it already works." },
    Value { icon: "phone", title: "Real numbers that answer", body: "Contact details are confirmed with hotel management, not lifted from a stale directory. If a number stops working, the listing comes down." },
];

const TIMELINE: &[(&str, &str, &str)] = &[
    ("2024", "Started in Addis Ababa", "Forty hotels in Bole and Kazanchis, verified on foot, with reservations coming in over WhatsApp."),
    ("2025", "Hotel dashboard launched", "Properties got their own admin tool for rooms, rates and reservations, replacing the shared spreadsheet."),
    ("2025", "Expanded across the Horn", "Mogadishu, Hargeisa and Djibouti City joined, followed by Dire Dawa and a full Nairobi catalogue."),
    ("2026", "690 hotels, six countries", "Nearly fifty thousand nights reserved, still with no booking fee and no card required."),
];

#[component]
pub fn AboutPage() -> impl IntoView {
    view! {
        <Title text="About us — Horn of Africa Hotel Portal" />

        <section class="relative overflow-hidden bg-slate-900">
            <img
                src="https://images.unsplash.com/photo-1553913861-c0fddf2619ee?q=80&w=2000"
                alt=""
                class="absolute inset-0 h-full w-full animate-kenburns object-cover opacity-35"
            />
            <div class="absolute inset-0 bg-gradient-to-br from-slate-950/90 via-slate-900/75 to-blue-900/50"></div>

            <div class="relative mx-auto max-w-4xl px-4 py-20 text-center text-white">
                <span class="inline-flex animate-fade-up items-center gap-2 rounded-full bg-white/10 px-3.5 py-1.5 text-xs font-bold uppercase tracking-wider ring-1 ring-white/20 backdrop-blur">
                    <Icon name="building" class="h-3.5 w-3.5" />
                    "About the portal"
                </span>
                <h1 class="mt-5 animate-fade-up text-4xl font-extrabold leading-tight tracking-tight sm:text-5xl" style="animation-delay: 80ms">
                    "Booking a hotel here should be "
                    <span class="bg-gradient-to-r from-sky-300 to-cyan-200 bg-clip-text text-transparent">"simple"</span>
                </h1>
                <p class="mx-auto mt-5 max-w-2xl animate-fade-up text-base leading-relaxed text-slate-200" style="animation-delay: 140ms">
                    "We built this because reserving a room across the Horn of Africa usually meant a phone tree, a WhatsApp thread and a lot of hoping. It should take a minute, cost nothing extra, and work without a credit card."
                </p>
            </div>
        </section>

        <section class="border-b border-slate-200 bg-white py-10">
            <div class="mx-auto max-w-6xl px-4">
                <div class="grid grid-cols-2 gap-6 lg:grid-cols-4">
                    {STATS.iter().enumerate().map(|(i, s)| {
                        let delay = format!("animation-delay: {}ms", i * 80);
                        view! {
                            <div class="reveal flex flex-col items-center gap-1.5 text-center" style=delay>
                                <span class="flex h-11 w-11 items-center justify-center rounded-2xl bg-blue-50 text-blue-700">
                                    <Icon name=s.icon class="h-5 w-5" />
                                </span>
                                <span class="text-3xl font-extrabold tracking-tight text-slate-900">{s.value}</span>
                                <span class="text-sm text-slate-500">{s.label}</span>
                            </div>
                        }
                    }).collect_view()}
                </div>
            </div>
        </section>

        <section class="mx-auto max-w-6xl px-4 py-16">
            <SectionHeading
                center=true
                eyebrow="What we stand for"
                title="Four things we will not compromise on"
                subtitle="These are the rules the portal was built around, and the reasons hotels and guests keep coming back."
            />

            <div class="mt-9 grid gap-5 md:grid-cols-2">
                {VALUES.iter().enumerate().map(|(i, v)| {
                    let delay = format!("animation-delay: {}ms", (i % 2) * 90);
                    view! {
                        <div class="reveal card-hover flex gap-4 rounded-2xl border border-slate-200 bg-white p-6" style=delay>
                            <span class="flex h-12 w-12 shrink-0 items-center justify-center rounded-2xl bg-blue-50 text-blue-700">
                                <Icon name=v.icon class="h-5 w-5" />
                            </span>
                            <div>
                                <h3 class="text-base font-bold text-slate-900">{v.title}</h3>
                                <p class="mt-1.5 text-sm leading-relaxed text-slate-600">{v.body}</p>
                            </div>
                        </div>
                    }
                }).collect_view()}
            </div>
        </section>

        <section class="border-y border-slate-200 bg-white py-16">
            <div class="mx-auto max-w-4xl px-4">
                <SectionHeading
                    eyebrow="How we got here"
                    title="A short history"
                    subtitle="Still a small team, still verifying properties in person."
                />

                <div class="relative mt-9 pl-8">
                    <span class="absolute bottom-6 left-[7px] top-2 w-0.5 bg-gradient-to-b from-blue-500 via-blue-300 to-transparent"></span>
                    {TIMELINE.iter().enumerate().map(|(i, (year, title, body))| {
                        let delay = format!("animation-delay: {}ms", i * 90);
                        view! {
                            <div class="reveal relative pb-9 last:pb-0" style=delay>
                                <span class="absolute -left-8 top-1.5 flex h-4 w-4 items-center justify-center rounded-full bg-blue-700 ring-4 ring-white">
                                    <span class="h-1.5 w-1.5 rounded-full bg-white"></span>
                                </span>
                                <span class="inline-flex rounded-full bg-blue-50 px-2.5 py-0.5 text-xs font-bold text-blue-700">{*year}</span>
                                <h3 class="mt-1.5 text-base font-bold text-slate-900">{*title}</h3>
                                <p class="mt-1 text-sm leading-relaxed text-slate-600">{*body}</p>
                            </div>
                        }
                    }).collect_view()}
                </div>
            </div>
        </section>

        <section class="mx-auto max-w-6xl px-4 py-16">
            <SectionHeading
                center=true
                eyebrow="The flow"
                title="What booking here actually looks like"
            />
            <div class="mt-9 grid gap-6 sm:grid-cols-2 lg:grid-cols-4">
                {HOW_IT_WORKS.iter().enumerate().map(|(i, step)| {
                    let delay = format!("animation-delay: {}ms", i * 80);
                    view! {
                        <div class="reveal group rounded-2xl border border-slate-200 bg-white p-5 transition-all duration-300 hover:-translate-y-1 hover:border-blue-200 hover:shadow-lg" style=delay>
                            <span class="flex h-11 w-11 items-center justify-center rounded-xl bg-blue-50 text-blue-700 transition-colors duration-300 group-hover:bg-blue-700 group-hover:text-white">
                                <Icon name=step.icon class="h-5 w-5" />
                            </span>
                            <p class="mt-3 text-xs font-bold text-blue-700">{step.number}</p>
                            <h3 class="text-base font-bold text-slate-900">{step.title}</h3>
                            <p class="mt-1 text-sm leading-relaxed text-slate-600">{step.body}</p>
                        </div>
                    }
                }).collect_view()}
            </div>

            <div class="mt-10">
                <TrustBar />
            </div>
        </section>

        <section class="mx-auto max-w-6xl px-4 pb-16">
            <div class="reveal grid gap-6 rounded-3xl border border-slate-200 bg-white p-8 lg:grid-cols-2 lg:items-center">
                <div>
                    <span class="inline-flex items-center gap-1.5 rounded-full bg-emerald-50 px-3 py-1 text-[11px] font-bold uppercase tracking-wider text-emerald-700 ring-1 ring-emerald-100">
                        <Icon name="building" class="h-3.5 w-3.5" />
                        "For hoteliers"
                    </span>
                    <h2 class="mt-3 text-2xl font-extrabold tracking-tight text-slate-900">"Run a hotel? List it here"</h2>
                    <p class="mt-2 text-sm leading-relaxed text-slate-600">
                        "You keep full control of rooms, rates and availability through the Tourista dashboard, and you keep the whole rate — we take no commission from the guest and none from you. Onboarding usually takes a couple of days including verification."
                    </p>
                    <A
                        href="/contact"
                        attr:class="sheen mt-5 inline-flex items-center gap-2 rounded-xl bg-blue-700 px-6 py-3 text-sm font-bold text-white shadow-lg shadow-blue-700/25 transition-all duration-200 hover:-translate-y-0.5 hover:bg-blue-800 hover:shadow-xl"
                    >
                        "Talk to our team"
                        <Icon name="arrow-right" class="h-4 w-4" />
                    </A>
                </div>
                <ul class="flex flex-col gap-3">
                    {[
                        ("check-circle", "Your own dashboard", "Rooms, rates, calendar, guests and reviews in one place."),
                        ("percent", "Zero commission", "Guests pay you directly at the front desk."),
                        ("bell", "Instant notifications", "Reservations land in the dashboard the moment they are made."),
                        ("shield-check", "Verified badge", "Verified listings rank ahead of unverified ones in search."),
                    ].into_iter().map(|(icon, title, body)| view! {
                        <li class="flex items-start gap-3 rounded-xl bg-slate-50 p-4">
                            <Icon name=icon class="mt-0.5 h-4.5 w-4.5 shrink-0 text-blue-700" />
                            <span>
                                <span class="block text-sm font-bold text-slate-800">{title}</span>
                                <span class="block text-xs leading-relaxed text-slate-500">{body}</span>
                            </span>
                        </li>
                    }).collect_view()}
                </ul>
            </div>
        </section>
    }
}
