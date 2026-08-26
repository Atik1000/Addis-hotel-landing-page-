//! Small presentational building blocks shared across pages.

use crate::components::Icon;
use leptos::prelude::*;

/// Row of five stars for a rating out of 5, with half-star support.
#[component]
pub fn Stars(
    rating: f32,
    #[prop(default = "h-3.5 w-3.5")] class: &'static str,
) -> impl IntoView {
    view! {
        <span class="inline-flex items-center gap-0.5 text-amber-500">
            {(1..=5).map(|i| {
                let i = i as f32;
                let name = if rating >= i {
                    "star"
                } else if rating >= i - 0.5 {
                    "star-half"
                } else {
                    "star-outline"
                };
                view! { <Icon name=name class=class /> }
            }).collect_view()}
        </span>
    }
}

/// Amber pill showing a numeric score, used on cards and detail headers.
#[component]
pub fn RatingBadge(
    rating: f32,
    review_count: u32,
    #[prop(default = false)] compact: bool,
) -> impl IntoView {
    let word = match rating {
        r if r >= 4.5 => "Exceptional",
        r if r >= 4.2 => "Excellent",
        r if r >= 3.8 => "Very good",
        r if r >= 3.4 => "Good",
        _ => "Pleasant",
    };
    view! {
        <span class="flex items-center gap-2">
            <span class="flex items-center gap-1 rounded-lg bg-amber-100 px-2 py-1 text-sm font-bold text-amber-700">
                <Icon name="star" class="h-3.5 w-3.5" />
                {format!("{rating:.1}")}
            </span>
            <Show when=move || !compact>
                <span class="text-sm">
                    <span class="font-semibold text-slate-700">{word}</span>
                    <span class="text-slate-400">{format!(" · {review_count} reviews")}</span>
                </span>
            </Show>
        </span>
    }
}

/// Centred (or left-aligned) section heading with an eyebrow label.
#[component]
pub fn SectionHeading(
    eyebrow: &'static str,
    title: &'static str,
    #[prop(default = "")] subtitle: &'static str,
    #[prop(default = false)] center: bool,
) -> impl IntoView {
    let align = if center { "text-center items-center" } else { "items-start" };
    view! {
        <div class=format!("reveal flex flex-col gap-2 {align}")>
            <span class="inline-flex items-center gap-1.5 rounded-full bg-blue-50 px-3 py-1 text-[11px] font-bold uppercase tracking-wider text-blue-700 ring-1 ring-blue-100">
                <span class="h-1.5 w-1.5 rounded-full bg-blue-600"></span>
                {eyebrow}
            </span>
            <h2 class="text-2xl font-extrabold tracking-tight text-slate-900 sm:text-3xl">{title}</h2>
            <Show when=move || !subtitle.is_empty()>
                <p class=format!("max-w-2xl text-sm leading-relaxed text-slate-500 sm:text-base {}", if center { "mx-auto" } else { "" })>
                    {subtitle}
                </p>
            </Show>
        </div>
    }
}

/// Single expandable question. Kept uncontrolled so several can be open at once.
#[component]
pub fn AccordionItem(
    question: &'static str,
    answer: &'static str,
    #[prop(default = false)] start_open: bool,
) -> impl IntoView {
    let open = RwSignal::new(start_open);
    view! {
        <div class=move || format!(
            "overflow-hidden rounded-xl border bg-white transition-all duration-300 {}",
            if open.get() { "border-blue-200 shadow-md shadow-blue-900/5" } else { "border-slate-200 hover:border-slate-300" }
        )>
            <button
                on:click=move |_| open.update(|v| *v = !*v)
                class="flex w-full items-center justify-between gap-4 px-5 py-4 text-left"
            >
                <span class=move || format!(
                    "text-sm font-semibold transition-colors sm:text-base {}",
                    if open.get() { "text-blue-700" } else { "text-slate-800" }
                )>{question}</span>
                <span class=move || format!(
                    "flex h-7 w-7 shrink-0 items-center justify-center rounded-full transition-all duration-300 {}",
                    if open.get() { "rotate-180 bg-blue-600 text-white" } else { "bg-slate-100 text-slate-500" }
                )>
                    <Icon name="chevron-down" class="h-4 w-4" />
                </span>
            </button>
            <div class=move || format!(
                "grid transition-all duration-300 ease-out {}",
                if open.get() { "grid-rows-[1fr] opacity-100" } else { "grid-rows-[0fr] opacity-0" }
            )>
                <div class="overflow-hidden">
                    <p class="px-5 pb-5 text-sm leading-relaxed text-slate-600">{answer}</p>
                </div>
            </div>
        </div>
    }
}

/// Illustrated placeholder for "nothing here yet" states.
#[component]
pub fn EmptyState(
    icon: &'static str,
    title: &'static str,
    body: &'static str,
    children: Children,
) -> impl IntoView {
    view! {
        <div class="flex animate-fade-up flex-col items-center gap-3 rounded-2xl border border-dashed border-slate-300 bg-slate-50/60 px-6 py-14 text-center">
            <span class="relative flex h-16 w-16 items-center justify-center rounded-full bg-white text-slate-400 shadow-sm ring-1 ring-slate-200">
                <span class="absolute inset-0 animate-pulse-ring rounded-full bg-slate-200"></span>
                <Icon name=icon class="relative h-7 w-7" />
            </span>
            <h3 class="text-base font-bold text-slate-800">{title}</h3>
            <p class="max-w-sm text-sm text-slate-500">{body}</p>
            <div class="mt-2">{children()}</div>
        </div>
    }
}

/// Trust strip: the four promises repeated across the site.
#[component]
pub fn TrustBar() -> impl IntoView {
    let items = [
        ("shield-check", "No booking fees", "The hotel's own rate"),
        ("wallet", "Pay at the hotel", "No card details needed"),
        ("check-circle", "Instant reservation", "Confirmed in seconds"),
        ("users", "No account required", "Name and number is enough"),
    ];
    view! {
        <div class="grid grid-cols-2 gap-3 lg:grid-cols-4">
            {items.into_iter().enumerate().map(|(i, (icon, title, hint))| {
                let delay = format!("animation-delay: {}ms", i * 70);
                view! {
                    <div
                        class="reveal group flex items-center gap-3 rounded-xl border border-slate-200 bg-white p-3.5 transition-all duration-300 hover:-translate-y-0.5 hover:border-blue-200 hover:shadow-md"
                        style=delay
                    >
                        <span class="flex h-10 w-10 shrink-0 items-center justify-center rounded-xl bg-blue-50 text-blue-700 transition-transform duration-300 group-hover:scale-110 group-hover:rotate-6">
                            <Icon name=icon class="h-5 w-5" />
                        </span>
                        <span class="min-w-0">
                            <span class="block truncate text-sm font-bold text-slate-800">{title}</span>
                            <span class="block truncate text-xs text-slate-500">{hint}</span>
                        </span>
                    </div>
                }
            }).collect_view()}
        </div>
    }
}

/// Breadcrumb trail. Each entry is (label, optional href).
#[component]
pub fn Breadcrumbs(trail: Vec<(String, Option<String>)>) -> impl IntoView {
    let last = trail.len().saturating_sub(1);
    view! {
        <nav class="flex flex-wrap items-center gap-1.5 text-sm text-slate-500">
            {trail.into_iter().enumerate().map(|(i, (label, href))| {
                let is_last = i == last;
                view! {
                    <span class="flex items-center gap-1.5">
                        {match href {
                            Some(h) if !is_last => view! {
                                <a href=h class="transition-colors hover:text-blue-700 hover:underline">{label.clone()}</a>
                            }.into_any(),
                            _ => view! {
                                <span class=if is_last { "font-semibold text-slate-800" } else { "" }>{label.clone()}</span>
                            }.into_any(),
                        }}
                        <Show when=move || !is_last>
                            <Icon name="chevron-right" class="h-3 w-3 text-slate-300" />
                        </Show>
                    </span>
                }
            }).collect_view()}
        </nav>
    }
}

/// Full-screen modal shell with a backdrop, used by the gallery and dialogs.
#[component]
pub fn Modal(
    #[prop(into)] title: String,
    on_close: impl Fn() + Copy + Send + Sync + 'static,
    #[prop(default = "max-w-lg")] width: &'static str,
    children: Children,
) -> impl IntoView {
    view! {
        <div class="fixed inset-0 z-50 flex items-center justify-center p-4">
            <div
                class="absolute inset-0 animate-fade-in bg-slate-900/60 backdrop-blur-sm"
                on:click=move |_| on_close()
            ></div>
            <div class=format!("relative max-h-[88vh] w-full {width} animate-scale-in overflow-y-auto rounded-2xl bg-white shadow-2xl")>
                <div class="sticky top-0 z-10 flex items-center justify-between gap-4 border-b border-slate-100 bg-white/95 px-6 py-4 backdrop-blur">
                    <h2 class="text-lg font-bold text-slate-900">{title}</h2>
                    <button
                        aria-label="Close"
                        class="flex h-9 w-9 items-center justify-center rounded-lg text-slate-400 transition-colors hover:bg-slate-100 hover:text-slate-700"
                        on:click=move |_| on_close()
                    >
                        <Icon name="x" class="h-5 w-5" />
                    </button>
                </div>
                <div class="px-6 py-5">{children()}</div>
            </div>
        </div>
    }
}
