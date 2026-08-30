//! `/my-reviews` — the reviews this guest has written, and editing them.
//!
//! Backed by `GET /reviews/my-reviews/` and `PATCH /reviews/{id}/`, both of
//! which need a signed-in guest, so the page asks for sign-in rather than
//! showing an empty state to someone who simply has no session yet.

use crate::api::{my_reviews, update_review, MyReview};
use crate::components::{use_toast, Icon, Stars};
use crate::session;
use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::components::A;

#[component]
pub fn MyReviewsPage() -> impl IntoView {
    let guest = session::use_guest();
    // Re-runs when the session appears at hydration, and again after a sign-out.
    let reviews = LocalResource::new(move || {
        let token = guest.get().map(|g| g.access).unwrap_or_default();
        async move {
            if token.is_empty() {
                return Ok(Vec::new());
            }
            my_reviews(token).await
        }
    });

    view! {
        <Title text="My reviews — Horn of Africa Hotel Portal" />

        <div class="mx-auto max-w-3xl px-4 py-8">
            <div class="flex flex-wrap items-end justify-between gap-3">
                <div>
                    <h1 class="text-2xl font-bold tracking-tight text-ink">"My reviews"</h1>
                    <p class="mt-1 text-sm text-slate-500">
                        "Everything you have written about the stays you have booked."
                    </p>
                </div>
                <Show when=move || guest.get().is_some()>
                    <button
                        on:click=move |_| session::sign_out()
                        class="rounded-lg border border-slate-300 px-3 py-2 text-xs font-bold text-slate-600 transition-colors hover:border-slate-400 hover:text-ink"
                    >
                        "Sign out"
                    </button>
                </Show>
            </div>

            // ---- Signed out ------------------------------------------------
            <Show when=move || guest.get().is_none()>
                <div class="mt-8 rounded-2xl border border-slate-200 bg-white px-5 py-10 text-center">
                    <span class="mx-auto mb-4 flex h-12 w-12 items-center justify-center rounded-2xl bg-blue-50 text-blue-700">
                        <Icon name="key" class="h-6 w-6" />
                    </span>
                    <h2 class="text-lg font-bold text-ink">"Sign in to see your reviews"</h2>
                    <p class="mx-auto mt-2 max-w-sm text-sm leading-relaxed text-slate-500">
                        "We will send a six-digit code to the email or phone you booked with. No password needed."
                    </p>
                    <A
                        href="/sign-in?next=/my-reviews"
                        attr:class="sheen mt-5 inline-flex items-center gap-2 rounded-xl bg-blue-700 px-5 py-2.5 text-sm font-bold text-white shadow-md transition-colors hover:bg-blue-800"
                    >
                        "Sign in"
                        <Icon name="arrow-right" class="h-4 w-4" />
                    </A>
                </div>
            </Show>

            // ---- Signed in -------------------------------------------------
            <Show when=move || guest.get().is_some()>
                <Suspense fallback=|| view! {
                    <div class="mt-7 flex flex-col gap-3">
                        {(0..2).map(|_| view! { <div class="skeleton h-32 rounded-2xl"></div> }).collect_view()}
                    </div>
                }>
                    {move || Suspend::new(async move {
                        match reviews.await {
                            Err(e) => view! {
                                <p class="mt-7 rounded-xl border border-red-200 bg-red-50 px-4 py-3 text-sm text-red-700">
                                    {e.to_string()}
                                </p>
                            }.into_any(),
                            Ok(list) if list.is_empty() => view! {
                                <div class="mt-7 rounded-2xl border border-slate-200 bg-white px-5 py-12 text-center">
                                    <span class="mx-auto mb-3 flex h-12 w-12 items-center justify-center rounded-full bg-slate-100 text-slate-400">
                                        <Icon name="star" class="h-6 w-6" />
                                    </span>
                                    <h2 class="text-base font-bold text-ink">"No reviews yet"</h2>
                                    <p class="mt-1.5 text-sm text-slate-500">
                                        "Once you have stayed somewhere, open the booking and tell us how it went."
                                    </p>
                                    <A
                                        href="/my-reservations"
                                        attr:class="mt-5 inline-flex items-center gap-1.5 text-sm font-bold text-blue-700 hover:underline"
                                    >
                                        "My reservations"
                                        <Icon name="arrow-right" class="h-3.5 w-3.5" />
                                    </A>
                                </div>
                            }.into_any(),
                            Ok(list) => view! {
                                <div class="mt-7 flex flex-col gap-4">
                                    {list.into_iter().map(|r| view! {
                                        <ReviewCard review=r on_saved=Callback::new(move |_| { reviews.refetch(); }) />
                                    }).collect_view()}
                                </div>
                            }.into_any(),
                        }
                    })}
                </Suspense>
            </Show>
        </div>
    }
}

/// One review, switching between a read view and an edit form in place.
#[component]
fn ReviewCard(review: MyReview, on_saved: Callback<()>) -> impl IntoView {
    let toast = use_toast();
    let guest = session::use_guest();

    let editing = RwSignal::new(false);
    let rate = RwSignal::new(review.rate.clamp(1, 5));
    let text = RwSignal::new(review.text());
    let busy = RwSignal::new(false);

    let id = review.id;
    let hotel = review.organization_name.clone();
    let reply = review.reply.clone();
    let has_reply = review.has_reply();
    let original_rate = review.rate;
    let original_text = StoredValue::new(review.text());

    let save = move |_| {
        if busy.get() {
            return;
        }
        let Some(token) = guest.get().map(|g| g.access) else {
            toast.error("Signed out", "Sign in again to edit this review.");
            return;
        };
        busy.set(true);
        let (r, t) = (rate.get(), text.get());
        leptos::task::spawn_local(async move {
            match update_review(token, id, r, t).await {
                Ok(_) => {
                    busy.set(false);
                    editing.set(false);
                    toast.success("Review updated", "Your changes are live.");
                    on_saved.run(());
                }
                Err(e) => {
                    busy.set(false);
                    toast.error("Could not save", e.to_string());
                }
            }
        });
    };

    view! {
        <article class="rounded-2xl border border-slate-200 bg-white p-5">
            <div class="flex flex-wrap items-start justify-between gap-3">
                <div class="min-w-0">
                    <h3 class="truncate text-base font-bold text-ink">{hotel}</h3>
                    <div class="mt-1 flex items-center gap-2">
                        <Stars rating=Signal::derive(move || rate.get() as f32) class="h-3.5 w-3.5" />
                        <span class="text-xs text-slate-400">
                            {move || format!("{} of 5", rate.get())}
                        </span>
                    </div>
                </div>
                <Show when=move || !editing.get()>
                    <button
                        on:click=move |_| editing.set(true)
                        class="flex items-center gap-1.5 rounded-lg border border-slate-300 px-3 py-1.5 text-xs font-bold text-slate-600 transition-colors hover:border-blue-300 hover:text-blue-700"
                    >
                        <Icon name="edit" class="h-3.5 w-3.5" />
                        "Edit"
                    </button>
                </Show>
            </div>

            // ---- Read view -------------------------------------------------
            <Show when=move || !editing.get()>
                <p class="mt-3 whitespace-pre-line text-sm leading-relaxed text-slate-600">
                    {move || {
                        let t = text.get();
                        if t.trim().is_empty() { "No comment left.".to_string() } else { t }
                    }}
                </p>
            </Show>

            // ---- Edit view -------------------------------------------------
            <Show when=move || editing.get()>
                <div class="mt-4 flex flex-col gap-3">
                    <StarPicker value=rate />
                    <textarea
                        rows="4"
                        class="w-full rounded-xl border border-slate-300 px-3.5 py-2.5 text-sm text-slate-800 transition-all duration-200 focus:border-blue-500 focus:outline-none focus:ring-4 focus:ring-blue-100"
                        placeholder="What stood out about the stay?"
                        prop:value=move || text.get()
                        on:input=move |e| text.set(event_target_value(&e))
                    ></textarea>
                    <div class="flex gap-2">
                        <button
                            on:click=save
                            disabled=move || busy.get()
                            class="rounded-xl bg-blue-700 px-4 py-2.5 text-sm font-bold text-white transition-colors hover:bg-blue-800 disabled:cursor-not-allowed disabled:opacity-60"
                        >
                            {move || if busy.get() { "Saving…" } else { "Save changes" }}
                        </button>
                        <button
                            on:click=move |_| {
                                // Abandon the edit rather than keeping half-typed text.
                                rate.set(original_rate.clamp(1, 5));
                                text.set(original_text.get_value());
                                editing.set(false);
                            }
                            class="rounded-xl border border-slate-300 px-4 py-2.5 text-sm font-bold text-slate-600 transition-colors hover:border-slate-400"
                        >
                            "Cancel"
                        </button>
                    </div>
                </div>
            </Show>

            // ---- The hotel's answer ----------------------------------------
            <Show when=move || has_reply>
                <div class="mt-4 rounded-xl border-l-2 border-blue-300 bg-slate-50 px-4 py-3">
                    <p class="text-[11px] font-bold uppercase tracking-wider text-slate-400">
                        "Reply from the hotel"
                    </p>
                    <p class="mt-1 whitespace-pre-line text-sm leading-relaxed text-slate-600">
                        {reply.clone().unwrap_or_default()}
                    </p>
                </div>
            </Show>
        </article>
    }
}

/// Five clickable stars.
///
/// `Icon` takes a static class, so the filled/empty state lives on a wrapping
/// span whose class is reactive, and the glyph itself is swapped by name.
#[component]
pub fn StarPicker(value: RwSignal<i32>) -> impl IntoView {
    view! {
        <div class="flex items-center gap-1.5" role="radiogroup" aria-label="Rating">
            {(1..=5).map(|i| view! {
                <button
                    type="button"
                    role="radio"
                    aria-checked=move || if value.get() == i { "true" } else { "false" }
                    aria-label=format!("{i} star{}", if i == 1 { "" } else { "s" })
                    on:click=move |_| value.set(i)
                    class="transition-transform duration-150 hover:scale-110 focus:outline-none focus-visible:ring-2 focus-visible:ring-blue-400"
                >
                    <span class=move || { if value.get() >= i {
                        "inline-flex text-amber-500"
                    } else {
                        "inline-flex text-slate-300"
                    } }>
                        {move || if value.get() >= i {
                            view! { <Icon name="star" class="h-6 w-6" /> }.into_any()
                        } else {
                            view! { <Icon name="star-outline" class="h-6 w-6" /> }.into_any()
                        }}
                    </span>
                </button>
            }).collect_view()}
            <span class="ml-1.5 text-sm font-semibold text-slate-500">
                {move || { match value.get() {
                    1 => "Poor",
                    2 => "Fair",
                    3 => "Good",
                    4 => "Very good",
                    _ => "Excellent",
                } }}
            </span>
        </div>
    }
}
