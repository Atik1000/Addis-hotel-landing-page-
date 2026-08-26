use crate::components::Icon;
use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::components::A;

#[component]
pub fn NotFoundPage() -> impl IntoView {
    view! {
        <Title text="Page not found — Horn of Africa Hotel Portal" />

        <div class="relative overflow-hidden">
            <span class="pointer-events-none absolute -left-24 top-10 h-72 w-72 rounded-full bg-blue-100 blur-3xl"></span>
            <span class="pointer-events-none absolute -right-20 bottom-0 h-72 w-72 rounded-full bg-cyan-100 blur-3xl"></span>

            <div class="relative mx-auto flex max-w-xl flex-col items-center px-4 py-24 text-center">
                <div class="relative mb-6 flex h-24 w-24 animate-float items-center justify-center rounded-3xl bg-white text-blue-700 shadow-xl shadow-blue-900/10 ring-1 ring-slate-200">
                    <Icon name="map" class="h-11 w-11" />
                    <span class="absolute -right-2 -top-2 flex h-9 w-9 items-center justify-center rounded-full bg-red-500 text-sm font-extrabold text-white shadow-lg">
                        "?"
                    </span>
                </div>

                <p class="animate-fade-up bg-gradient-to-r from-blue-700 to-indigo-700 bg-clip-text text-7xl font-black tracking-tighter text-transparent">
                    "404"
                </p>
                <h1 class="mt-2 animate-fade-up text-2xl font-extrabold tracking-tight text-slate-900" style="animation-delay: 90ms">
                    "This page has checked out"
                </h1>
                <p class="mt-2 max-w-sm animate-fade-up text-sm leading-relaxed text-slate-500" style="animation-delay: 140ms">
                    "The link may be out of date, or the hotel listing has been removed. Let's get you back somewhere useful."
                </p>

                <div class="mt-7 flex animate-fade-up flex-wrap justify-center gap-2.5" style="animation-delay: 190ms">
                    <A
                        href="/"
                        attr:class="sheen flex items-center gap-2 rounded-xl bg-blue-700 px-6 py-3 text-sm font-bold text-white shadow-lg shadow-blue-700/25 transition-all duration-200 hover:-translate-y-0.5 hover:bg-blue-800 hover:shadow-xl active:scale-[0.98]"
                    >
                        <Icon name="home" class="h-4 w-4" />
                        "Back to home"
                    </A>
                    <A
                        href="/hotels"
                        attr:class="flex items-center gap-2 rounded-xl border border-slate-300 bg-white px-6 py-3 text-sm font-bold text-slate-700 transition-all duration-200 hover:-translate-y-0.5 hover:border-blue-300 hover:text-blue-700 hover:shadow-md"
                    >
                        <Icon name="search" class="h-4 w-4" />
                        "Browse hotels"
                    </A>
                </div>

                <div class="mt-10 w-full animate-fade-up rounded-2xl border border-slate-200 bg-white p-5 text-left" style="animation-delay: 240ms">
                    <h2 class="text-xs font-bold uppercase tracking-wider text-slate-400">"Popular pages"</h2>
                    <div class="mt-3 grid gap-2 sm:grid-cols-2">
                        {[
                            ("building", "All hotels", "/hotels"),
                            ("ticket", "My reservation", "/my-reservations"),
                            ("search", "Retrieve a booking", "/retrieve-booking"),
                            ("help-circle", "Help centre", "/faq"),
                        ].into_iter().map(|(icon, label, href)| view! {
                            <A
                                href=href
                                attr:class="group flex items-center gap-2.5 rounded-xl border border-slate-200 px-3.5 py-3 text-sm font-semibold text-slate-700 transition-all duration-200 hover:-translate-y-0.5 hover:border-blue-300 hover:bg-blue-50/50 hover:text-blue-700"
                            >
                                <Icon name=icon class="h-4 w-4 text-slate-400 transition-colors group-hover:text-blue-700" />
                                {label}
                                <Icon name="arrow-right" class="ml-auto h-3.5 w-3.5 text-slate-300 transition-transform duration-200 group-hover:translate-x-0.5" />
                            </A>
                        }).collect_view()}
                    </div>
                </div>
            </div>
        </div>
    }
}
