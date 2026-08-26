use crate::components::{
    thousands, AccordionItem, HotelCardCompact, Icon, SearchWidget, SectionHeading, Stars, TrustBar,
};
use crate::data::{featured_hotels, CITIES, FAQS, HOW_IT_WORKS, STATS, TESTIMONIALS};
use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::components::A;

#[component]
pub fn HomePage() -> impl IntoView {
    let featured = featured_hotels();
    let cheapest = featured.iter().map(|h| h.price_from).min().unwrap_or(0);

    view! {
        <Title text="Horn of Africa Hotel Portal — Discover & Reserve Hotels" />

        // ================= HERO =================
        <section class="relative overflow-hidden">
            <img
                src="https://images.unsplash.com/photo-1566073771259-6a8506099945?q=80&w=2000"
                alt=""
                class="absolute inset-0 h-full w-full animate-kenburns object-cover"
            />
            <div class="absolute inset-0 bg-gradient-to-br from-slate-950/90 via-slate-900/70 to-blue-900/40"></div>
            <div class="absolute inset-0 bg-[radial-gradient(ellipse_at_top_right,rgba(56,189,248,0.18),transparent_55%)]"></div>

            <div class="relative mx-auto grid max-w-6xl gap-10 px-4 py-14 lg:grid-cols-[1.05fr_minmax(0,26rem)] lg:items-center lg:py-20">
                <div class="animate-fade-up text-white">
                    <span class="inline-flex items-center gap-2 rounded-full bg-white/10 px-3.5 py-1.5 text-xs font-semibold ring-1 ring-white/20 backdrop-blur">
                        <span class="relative flex h-2 w-2">
                            <span class="absolute inline-flex h-full w-full animate-pulse-ring rounded-full bg-emerald-400"></span>
                            <span class="relative inline-flex h-2 w-2 rounded-full bg-emerald-400"></span>
                        </span>
                        "690+ verified hotels · 6 countries"
                    </span>

                    <h1 class="mt-5 text-4xl font-extrabold leading-[1.08] tracking-tight sm:text-5xl lg:text-6xl">
                        "Discover & reserve hotels across the "
                        <span class="animate-gradient-pan bg-gradient-to-r from-sky-300 via-cyan-200 to-blue-300 bg-clip-text text-transparent">
                            "Horn of Africa"
                        </span>
                    </h1>

                    <p class="mt-5 max-w-lg text-base leading-relaxed text-slate-200 sm:text-lg">
                        "Find a great hotel, reserve it in seconds and pay at the front desk. No account, no card details, no booking fees — ever."
                    </p>

                    <div class="mt-7 flex flex-wrap items-center gap-3">
                        <A
                            href="/hotels"
                            attr:class="sheen flex items-center gap-2 rounded-xl bg-white px-6 py-3.5 text-sm font-bold text-slate-900 shadow-xl transition-all duration-200 hover:-translate-y-0.5 hover:shadow-2xl active:scale-[0.98]"
                        >
                            <Icon name="search" class="h-4 w-4" />
                            "Browse all hotels"
                        </A>
                        <A
                            href="/how-it-works"
                            attr:class="flex items-center gap-2 rounded-xl border border-white/25 px-6 py-3.5 text-sm font-bold text-white backdrop-blur transition-all duration-200 hover:-translate-y-0.5 hover:bg-white/10"
                        >
                            <Icon name="sparkles" class="h-4 w-4" />
                            "How it works"
                        </A>
                    </div>

                    <div class="mt-9 flex flex-wrap items-center gap-x-8 gap-y-4">
                        <div class="flex items-center gap-3">
                            <div class="flex -space-x-2.5">
                                {["AH", "GW", "OF", "FA"].iter().enumerate().map(|(i, initials)| {
                                    let tints = ["bg-blue-500", "bg-emerald-500", "bg-amber-500", "bg-purple-500"];
                                    view! {
                                        <span class=format!(
                                            "flex h-9 w-9 items-center justify-center rounded-full text-[11px] font-bold text-white ring-2 ring-slate-900/40 {}",
                                            tints[i]
                                        )>{*initials}</span>
                                    }
                                }).collect_view()}
                            </div>
                            <div class="text-sm">
                                <Stars rating=4.6 class="h-3.5 w-3.5" />
                                <p class="mt-0.5 text-slate-300">"48,000+ nights reserved"</p>
                            </div>
                        </div>

                        <div class="text-sm text-slate-300">
                            <p class="text-xs uppercase tracking-wide text-slate-400">"Rooms from"</p>
                            <p class="text-xl font-extrabold text-white">{format!("ETB {}", thousands(cheapest))}
                                <span class="text-sm font-medium text-slate-300">" / night"</span>
                            </p>
                        </div>
                    </div>
                </div>

                <div class="animate-fade-up lg:animate-slide-in-right" style="animation-delay: 180ms">
                    <SearchWidget />
                </div>
            </div>

            <div class="relative border-t border-white/10 bg-slate-950/40 backdrop-blur">
                <div class="mx-auto max-w-6xl px-4 py-4">
                    <div class="flex flex-wrap items-center justify-center gap-x-8 gap-y-3 text-xs font-semibold text-slate-300 sm:text-sm">
                        <span class="flex items-center gap-2"><Icon name="shield-check" class="h-4 w-4 text-emerald-400" />"No booking fees"</span>
                        <span class="flex items-center gap-2"><Icon name="wallet" class="h-4 w-4 text-sky-400" />"Pay at the hotel"</span>
                        <span class="flex items-center gap-2"><Icon name="check-circle" class="h-4 w-4 text-emerald-400" />"Instant confirmation"</span>
                        <span class="flex items-center gap-2"><Icon name="users" class="h-4 w-4 text-amber-400" />"No account required"</span>
                    </div>
                </div>
            </div>
        </section>

        // ================= POPULAR CITIES =================
        <section class="mx-auto max-w-6xl px-4 py-14">
            <div class="mb-7 flex flex-wrap items-end justify-between gap-4">
                <SectionHeading
                    eyebrow="Destinations"
                    title="Popular cities"
                    subtitle="Six countries, one portal. Every property is verified before it goes live."
                />
                <A
                    href="/hotels"
                    attr:class="group flex shrink-0 items-center gap-1.5 text-sm font-bold text-blue-700 transition-colors hover:text-blue-800"
                >
                    "View all destinations"
                    <Icon name="arrow-right" class="h-3.5 w-3.5 transition-transform duration-200 group-hover:translate-x-1" />
                </A>
            </div>

            <div class="grid grid-cols-2 gap-4 sm:grid-cols-3 lg:grid-cols-6">
                {CITIES.iter().enumerate().map(|(i, c)| {
                    let delay = format!("animation-delay: {}ms", i * 60);
                    let href = format!("/hotels?city={}", c.name.replace(' ', "+"));
                    view! {
                        <a href=href class="reveal group relative block h-40 overflow-hidden rounded-2xl shadow-sm transition-shadow duration-300 hover:shadow-xl" style=delay>
                            <img
                                src=c.image
                                alt=c.name
                                loading="lazy"
                                class="h-full w-full object-cover transition-transform duration-700 group-hover:scale-110"
                            />
                            <span class="absolute inset-0 bg-gradient-to-t from-slate-950/85 via-slate-950/25 to-transparent transition-opacity duration-300 group-hover:from-blue-950/85"></span>
                            <span class="absolute inset-x-3 bottom-3 text-left text-white transition-transform duration-300 group-hover:-translate-y-1">
                                <span class="block text-sm font-bold">{c.name}</span>
                                <span class="block text-[11px] text-slate-300">{c.country}</span>
                                <span class="mt-1.5 inline-flex items-center gap-1 rounded-full bg-white/15 px-2 py-0.5 text-[10px] font-semibold backdrop-blur">
                                    {format!("{} hotels", c.hotel_count)}
                                </span>
                            </span>
                        </a>
                    }
                }).collect_view()}
            </div>
        </section>

        // ================= FEATURED HOTELS =================
        <section class="border-y border-slate-200 bg-white py-14">
            <div class="mx-auto max-w-6xl px-4">
                <div class="mb-7 flex flex-wrap items-end justify-between gap-4">
                    <SectionHeading
                        eyebrow="Hand-picked"
                        title="Featured hotels this month"
                        subtitle="Properties our team has stayed in, rated highly by guests and confirmed to honour portal rates."
                    />
                    <A
                        href="/hotels"
                        attr:class="group flex shrink-0 items-center gap-1.5 text-sm font-bold text-blue-700 transition-colors hover:text-blue-800"
                    >
                        "See all hotels"
                        <Icon name="arrow-right" class="h-3.5 w-3.5 transition-transform duration-200 group-hover:translate-x-1" />
                    </A>
                </div>

                <div class="grid gap-5 sm:grid-cols-2 lg:grid-cols-3">
                    {featured.into_iter().take(6).enumerate().map(|(i, h)| {
                        let delay = format!("animation-delay: {}ms", i * 70);
                        view! {
                            <div class="reveal" style=delay>
                                <HotelCardCompact hotel=h />
                            </div>
                        }
                    }).collect_view()}
                </div>
            </div>
        </section>

        // ================= HOW IT WORKS =================
        <section class="mx-auto max-w-6xl px-4 py-16">
            <SectionHeading
                center=true
                eyebrow="Four steps"
                title="Reserving a room takes under a minute"
                subtitle="No sign-up wall, no card form, no waiting for a confirmation email that never arrives."
            />

            <div class="relative mt-10">
                // Connector line behind the step cards on wide screens.
                <span class="pointer-events-none absolute left-[12%] right-[12%] top-9 hidden h-0.5 bg-gradient-to-r from-blue-200 via-blue-300 to-blue-200 lg:block"></span>

                <div class="grid gap-6 sm:grid-cols-2 lg:grid-cols-4">
                    {HOW_IT_WORKS.iter().enumerate().map(|(i, step)| {
                        let delay = format!("animation-delay: {}ms", i * 100);
                        view! {
                            <div class="reveal group relative flex flex-col items-center text-center" style=delay>
                                <span class="relative z-10 flex h-[4.5rem] w-[4.5rem] items-center justify-center rounded-2xl border border-blue-100 bg-white text-blue-700 shadow-md shadow-blue-900/5 transition-all duration-300 group-hover:-translate-y-1.5 group-hover:border-blue-300 group-hover:bg-blue-700 group-hover:text-white group-hover:shadow-xl">
                                    <Icon name=step.icon class="h-7 w-7" />
                                    <span class="absolute -right-2 -top-2 flex h-7 w-7 items-center justify-center rounded-full bg-blue-700 text-[11px] font-extrabold text-white ring-4 ring-slate-50 transition-colors duration-300 group-hover:bg-slate-900">
                                        {step.number}
                                    </span>
                                </span>
                                <h3 class="mt-4 text-base font-bold text-slate-900">{step.title}</h3>
                                <p class="mt-1.5 text-sm leading-relaxed text-slate-500">{step.body}</p>
                            </div>
                        }
                    }).collect_view()}
                </div>
            </div>

            <div class="mt-10">
                <TrustBar />
            </div>
        </section>

        // ================= STATS =================
        <section class="relative overflow-hidden bg-slate-900 py-14">
            <span class="pointer-events-none absolute -left-24 top-0 h-72 w-72 rounded-full bg-blue-600/20 blur-3xl"></span>
            <span class="pointer-events-none absolute -right-24 bottom-0 h-72 w-72 rounded-full bg-cyan-500/15 blur-3xl"></span>

            <div class="relative mx-auto max-w-6xl px-4">
                <div class="grid grid-cols-2 gap-6 lg:grid-cols-4">
                    {STATS.iter().enumerate().map(|(i, s)| {
                        let delay = format!("animation-delay: {}ms", i * 90);
                        view! {
                            <div class="reveal group flex flex-col items-center gap-2 text-center text-white" style=delay>
                                <span class="flex h-12 w-12 items-center justify-center rounded-2xl bg-white/10 text-sky-300 ring-1 ring-white/15 transition-all duration-300 group-hover:scale-110 group-hover:bg-white/15">
                                    <Icon name=s.icon class="h-5 w-5" />
                                </span>
                                <span class="text-3xl font-extrabold tracking-tight sm:text-4xl">{s.value}</span>
                                <span class="text-sm text-slate-400">{s.label}</span>
                            </div>
                        }
                    }).collect_view()}
                </div>
            </div>
        </section>

        // ================= TESTIMONIALS =================
        <section class="mx-auto max-w-6xl px-4 py-16">
            <SectionHeading
                center=true
                eyebrow="Guest stories"
                title="Trusted by travellers across the region"
                subtitle="Business travellers, NGO staff, diaspora families and holidaymakers — all booking the same way."
            />

            <div class="mt-9 grid gap-5 md:grid-cols-2 lg:grid-cols-3">
                {TESTIMONIALS.iter().enumerate().map(|(i, t)| {
                    let delay = format!("animation-delay: {}ms", (i % 3) * 90);
                    view! {
                        <figure
                            class="reveal card-hover relative flex h-full flex-col rounded-2xl border border-slate-200 bg-white p-6 shadow-sm"
                            style=delay
                        >
                            <Icon name="quote" class="h-7 w-7 text-blue-100" />
                            <blockquote class="mt-3 flex-1 text-sm leading-relaxed text-slate-600">
                                {t.quote}
                            </blockquote>
                            <div class="mt-4">
                                <Stars rating={t.rating as f32} class="h-3.5 w-3.5" />
                            </div>
                            <figcaption class="mt-4 flex items-center gap-3 border-t border-slate-100 pt-4">
                                <span class=format!(
                                    "flex h-10 w-10 shrink-0 items-center justify-center rounded-full text-xs font-bold {}",
                                    t.avatar_tint
                                )>{t.initials}</span>
                                <span class="min-w-0">
                                    <span class="block truncate text-sm font-bold text-slate-900">{t.name}</span>
                                    <span class="block truncate text-xs text-slate-400">{t.role}</span>
                                </span>
                            </figcaption>
                        </figure>
                    }
                }).collect_view()}
            </div>
        </section>

        // ================= FAQ =================
        <section class="border-t border-slate-200 bg-white py-16">
            <div class="mx-auto max-w-4xl px-4">
                <SectionHeading
                    center=true
                    eyebrow="Questions"
                    title="Everything you might be wondering"
                    subtitle="Still stuck? The full help centre covers cancellations, group bookings and listing your own property."
                />

                <div class="mt-8 flex flex-col gap-3">
                    {FAQS.iter().take(6).enumerate().map(|(i, f)| {
                        let delay = format!("animation-delay: {}ms", i * 50);
                        view! {
                            <div class="reveal" style=delay>
                                <AccordionItem question=f.question answer=f.answer start_open=(i == 0) />
                            </div>
                        }
                    }).collect_view()}
                </div>

                <div class="mt-7 text-center">
                    <A
                        href="/faq"
                        attr:class="group inline-flex items-center gap-2 rounded-xl border border-slate-300 px-5 py-3 text-sm font-bold text-slate-700 transition-all duration-200 hover:-translate-y-0.5 hover:border-blue-300 hover:text-blue-700 hover:shadow-md"
                    >
                        "Visit the help centre"
                        <Icon name="arrow-right" class="h-3.5 w-3.5 transition-transform duration-200 group-hover:translate-x-1" />
                    </A>
                </div>
            </div>
        </section>

        // ================= CLOSING CTA =================
        <section class="mx-auto max-w-6xl px-4 py-16">
            <div class="reveal relative overflow-hidden rounded-3xl bg-gradient-to-br from-slate-900 via-blue-900 to-indigo-900 px-6 py-14 text-center text-white shadow-2xl shadow-blue-950/30 sm:px-12">
                <span class="pointer-events-none absolute -left-20 -top-20 h-64 w-64 rounded-full bg-cyan-400/15 blur-3xl"></span>
                <span class="pointer-events-none absolute -bottom-24 -right-16 h-72 w-72 rounded-full bg-blue-500/20 blur-3xl"></span>

                <span class="relative mx-auto flex h-14 w-14 animate-float items-center justify-center rounded-2xl bg-white/10 ring-1 ring-white/20 backdrop-blur">
                    <Icon name="plane" class="h-6 w-6 text-sky-300" />
                </span>

                <h2 class="relative mt-5 text-3xl font-extrabold tracking-tight sm:text-4xl">
                    "Your next stay is a minute away"
                </h2>
                <p class="relative mx-auto mt-3 max-w-xl text-sm leading-relaxed text-blue-100 sm:text-base">
                    "Search verified hotels, reserve without a card and settle up at the front desk. It really is that simple."
                </p>

                <div class="relative mt-7 flex flex-wrap items-center justify-center gap-3">
                    <A
                        href="/hotels"
                        attr:class="sheen flex items-center gap-2 rounded-xl bg-white px-7 py-3.5 text-sm font-bold text-slate-900 shadow-xl transition-all duration-200 hover:-translate-y-0.5 hover:shadow-2xl active:scale-[0.98]"
                    >
                        <Icon name="search" class="h-4 w-4" />
                        "Start searching"
                    </A>
                    <A
                        href="/retrieve-booking"
                        attr:class="flex items-center gap-2 rounded-xl border border-white/25 px-7 py-3.5 text-sm font-bold text-white transition-all duration-200 hover:-translate-y-0.5 hover:bg-white/10"
                    >
                        <Icon name="ticket" class="h-4 w-4" />
                        "Retrieve a booking"
                    </A>
                </div>
            </div>
        </section>
    }
}
