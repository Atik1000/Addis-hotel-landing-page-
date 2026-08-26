//! Hotel photo mosaic with a full-screen lightbox.

use crate::components::Icon;
use leptos::prelude::*;

#[component]
pub fn Gallery(photos: &'static [&'static str], alt: &'static str) -> impl IntoView {
    let open = RwSignal::new(false);
    let index = RwSignal::new(0usize);
    let count = photos.len();

    let show = move |i: usize| {
        index.set(i);
        open.set(true);
    };
    let next = move |_| index.update(|i| *i = (*i + 1) % count);
    let prev = move |_| index.update(|i| *i = (*i + count - 1) % count);

    // The mosaic shows a hero plus up to four thumbnails; the last thumbnail
    // carries the "+N photos" overlay when there are more than five.
    let thumbs: Vec<(usize, &'static str)> =
        photos.iter().skip(1).take(4).copied().enumerate().map(|(i, p)| (i + 1, p)).collect();
    let hidden = count.saturating_sub(5);
    let last_thumb = thumbs.len().saturating_sub(1);

    view! {
        <div class="relative">
            <div class="grid h-[22rem] grid-cols-4 grid-rows-2 gap-2 overflow-hidden rounded-2xl sm:h-[26rem]">
                <button
                    on:click=move |_| show(0)
                    class="group relative col-span-4 row-span-2 overflow-hidden sm:col-span-2"
                >
                    <img
                        src=photos[0]
                        alt=alt
                        class="h-full w-full object-cover transition-transform duration-700 group-hover:scale-105"
                    />
                    <span class="absolute inset-0 bg-gradient-to-t from-slate-900/40 via-transparent to-transparent opacity-0 transition-opacity duration-300 group-hover:opacity-100"></span>
                </button>

                {thumbs.into_iter().map(|(i, p)| {
                    let is_last = i == last_thumb + 1;
                    view! {
                        <button
                            on:click=move |_| show(i)
                            class="group relative hidden overflow-hidden sm:block"
                        >
                            <img
                                src=p
                                alt=""
                                class="h-full w-full object-cover transition-transform duration-700 group-hover:scale-105"
                            />
                            <Show when=move || is_last && (hidden > 0)>
                                <span class="absolute inset-0 flex flex-col items-center justify-center gap-1 bg-slate-900/60 text-white backdrop-blur-[2px] transition-colors group-hover:bg-slate-900/70">
                                    <Icon name="camera" class="h-5 w-5" />
                                    <span class="text-sm font-bold">{format!("+{hidden} photos")}</span>
                                </span>
                            </Show>
                        </button>
                    }
                }).collect_view()}
            </div>

            <button
                on:click=move |_| show(0)
                class="absolute bottom-3 right-3 flex items-center gap-1.5 rounded-lg bg-white/95 px-3 py-2 text-xs font-bold text-slate-800 shadow-lg backdrop-blur transition-all duration-200 hover:-translate-y-0.5 hover:shadow-xl"
            >
                <Icon name="camera" class="h-4 w-4" />
                {format!("View all {count} photos")}
            </button>

            // ---- Lightbox ---------------------------------------------------
            <Show when=move || open.get()>
                <div class="fixed inset-0 z-[60] flex flex-col bg-slate-950/95 backdrop-blur-sm animate-fade-in">
                    <div class="flex items-center justify-between px-4 py-3 text-white sm:px-6">
                        <span class="text-sm font-semibold">
                            {move || format!("{} / {}", index.get() + 1, count)}
                            <span class="ml-2 hidden text-slate-400 sm:inline">{alt}</span>
                        </span>
                        <button
                            aria-label="Close gallery"
                            class="flex h-10 w-10 items-center justify-center rounded-lg text-white/80 transition-colors hover:bg-white/10 hover:text-white"
                            on:click=move |_| open.set(false)
                        >
                            <Icon name="x" class="h-5 w-5" />
                        </button>
                    </div>

                    <div class="relative flex flex-1 items-center justify-center overflow-hidden px-4 pb-4">
                        <button
                            aria-label="Previous photo"
                            on:click=prev
                            class="absolute left-3 z-10 flex h-11 w-11 items-center justify-center rounded-full bg-white/10 text-white backdrop-blur transition-all hover:scale-110 hover:bg-white/20 sm:left-6"
                        >
                            <Icon name="chevron-left" class="h-5 w-5" />
                        </button>

                        {move || view! {
                            <img
                                src=photos[index.get()]
                                alt=""
                                class="max-h-full max-w-full animate-scale-in rounded-xl object-contain shadow-2xl"
                            />
                        }}

                        <button
                            aria-label="Next photo"
                            on:click=next
                            class="absolute right-3 z-10 flex h-11 w-11 items-center justify-center rounded-full bg-white/10 text-white backdrop-blur transition-all hover:scale-110 hover:bg-white/20 sm:right-6"
                        >
                            <Icon name="chevron-right" class="h-5 w-5" />
                        </button>
                    </div>

                    <div class="no-scrollbar flex gap-2 overflow-x-auto px-4 pb-5 sm:justify-center sm:px-6">
                        {photos.iter().enumerate().map(|(i, p)| view! {
                            <button
                                on:click=move |_| index.set(i)
                                class=move || format!(
                                    "h-14 w-20 shrink-0 overflow-hidden rounded-lg ring-2 transition-all duration-200 {}",
                                    if index.get() == i { "ring-white opacity-100 scale-105" } else { "ring-transparent opacity-50 hover:opacity-90" }
                                )
                            >
                                <img src=*p alt="" class="h-full w-full object-cover" />
                            </button>
                        }).collect_view()}
                    </div>
                </div>
            </Show>
        </div>
    }
}
