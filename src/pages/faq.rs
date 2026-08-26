use crate::components::{AccordionItem, EmptyState, Icon, SectionHeading};
use crate::data::FAQS;
use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::components::A;

const CATEGORIES: [(&str, &str); 5] = [
    ("All", "list"),
    ("Booking", "calendar-check"),
    ("Payment", "wallet"),
    ("Changes", "edit"),
    ("Hotels", "building"),
];

#[component]
pub fn FaqPage() -> impl IntoView {
    let category = RwSignal::new("All");
    let search = RwSignal::new(String::new());

    let visible = move || {
        let q = search.get().to_lowercase();
        let c = category.get();
        FAQS.iter()
            .filter(|f| c == "All" || f.category == c)
            .filter(|f| {
                q.is_empty()
                    || f.question.to_lowercase().contains(&q)
                    || f.answer.to_lowercase().contains(&q)
            })
            .collect::<Vec<_>>()
    };

    view! {
        <Title text="Help centre — Horn of Africa Hotel Portal" />

        <section class="relative overflow-hidden border-b border-slate-200 bg-gradient-to-br from-blue-50 via-white to-indigo-50">
            <span class="pointer-events-none absolute -right-20 -top-24 h-64 w-64 rounded-full bg-blue-200/40 blur-3xl"></span>

            <div class="relative mx-auto max-w-2xl px-4 py-14 text-center">
                <span class="mx-auto flex h-14 w-14 animate-scale-in items-center justify-center rounded-2xl bg-blue-700 text-white shadow-xl shadow-blue-700/25">
                    <Icon name="help-circle" class="h-6 w-6" />
                </span>
                <h1 class="mt-5 animate-fade-up text-3xl font-extrabold tracking-tight text-slate-900 sm:text-4xl" style="animation-delay: 70ms">
                    "How can we help?"
                </h1>
                <p class="mt-3 animate-fade-up text-sm leading-relaxed text-slate-600" style="animation-delay: 120ms">
                    "Answers about reserving, paying, changing plans and listing a property."
                </p>

                <div class="relative mt-6 animate-fade-up" style="animation-delay: 170ms">
                    <Icon name="search" class="pointer-events-none absolute left-4 top-1/2 h-4 w-4 -translate-y-1/2 text-slate-400" />
                    <input
                        type="text"
                        placeholder="Search the help centre…"
                        class="w-full rounded-2xl border border-slate-300 bg-white py-4 pl-11 pr-4 text-sm shadow-lg shadow-slate-900/5 transition-all duration-200 focus:border-blue-500 focus:outline-none focus:ring-4 focus:ring-blue-100"
                        prop:value=search
                        on:input:target=move |ev| search.set(ev.target().value())
                    />
                    <Show when=move || !search.get().is_empty()>
                        <button
                            aria-label="Clear search"
                            class="absolute right-3.5 top-1/2 -translate-y-1/2 rounded-lg p-1.5 text-slate-400 transition-colors hover:bg-slate-100 hover:text-slate-700"
                            on:click=move |_| search.set(String::new())
                        >
                            <Icon name="x" class="h-4 w-4" />
                        </button>
                    </Show>
                </div>
            </div>
        </section>

        <section class="mx-auto max-w-3xl px-4 py-10">
            <div class="no-scrollbar mb-6 flex gap-2 overflow-x-auto pb-1">
                {CATEGORIES.into_iter().map(|(name, icon)| view! {
                    <button
                        on:click=move |_| category.set(name)
                        class=move || format!(
                            "flex shrink-0 items-center gap-1.5 rounded-xl px-4 py-2.5 text-sm font-semibold transition-all duration-200 {}",
                            if category.get() == name {
                                "bg-blue-700 text-white shadow-md shadow-blue-700/25"
                            } else {
                                "border border-slate-300 bg-white text-slate-600 hover:-translate-y-0.5 hover:border-blue-300 hover:text-blue-700"
                            }
                        )
                    >
                        <Icon name=icon class="h-4 w-4" />
                        {name}
                    </button>
                }).collect_view()}
            </div>

            <Show
                when=move || !visible().is_empty()
                fallback=move || view! {
                    <EmptyState
                        icon="search"
                        title="Nothing matched that"
                        body="Try a different word, or ask our team directly — we usually reply within a few hours."
                    >
                        <A
                            href="/contact"
                            attr:class="rounded-xl bg-blue-700 px-5 py-2.5 text-sm font-bold text-white transition-colors hover:bg-blue-800"
                        >
                            "Contact support"
                        </A>
                    </EmptyState>
                }
            >
                <div class="flex flex-col gap-3">
                    {move || visible().into_iter().enumerate().map(|(i, f)| {
                        let delay = format!("animation-delay: {}ms", i * 40);
                        view! {
                            <div class="animate-fade-up" style=delay>
                                <AccordionItem question=f.question answer=f.answer />
                            </div>
                        }
                    }).collect_view()}
                </div>
            </Show>

            <p class="mt-4 text-center text-sm text-slate-400">
                {move || format!("{} of {} articles", visible().len(), FAQS.len())}
            </p>
        </section>

        <section class="mx-auto max-w-4xl px-4 pb-16">
            <SectionHeading center=true eyebrow="Still stuck?" title="Reach a human" />
            <div class="mt-7 grid gap-4 sm:grid-cols-3">
                {[
                    ("headset", "Contact form", "Tell us what happened and we'll pick it up.", "/contact"),
                    ("ticket", "Retrieve a booking", "Lost your reference? Search with your contact details.", "/retrieve-booking"),
                    ("building", "List your hotel", "Get your property onto the portal.", "/contact"),
                ].into_iter().enumerate().map(|(i, (icon, title, body, href))| {
                    let delay = format!("animation-delay: {}ms", i * 80);
                    view! {
                        <A href=href attr:class="reveal group block h-full" attr:style=delay>
                            <div class="card-hover flex h-full flex-col items-center gap-2 rounded-2xl border border-slate-200 bg-white p-6 text-center">
                                <span class="flex h-12 w-12 items-center justify-center rounded-2xl bg-blue-50 text-blue-700 transition-colors duration-300 group-hover:bg-blue-700 group-hover:text-white">
                                    <Icon name=icon class="h-5 w-5" />
                                </span>
                                <h3 class="mt-1 text-sm font-bold text-slate-900">{title}</h3>
                                <p class="text-xs leading-relaxed text-slate-500">{body}</p>
                            </div>
                        </A>
                    }
                }).collect_view()}
            </div>
        </section>
    }
}
