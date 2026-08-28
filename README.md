# Addis-hotel-landing-page-

Guest-facing hotel portal for the Horn of Africa — discover and reserve verified
hotels across Ethiopia, Kenya, Somalia, Somaliland and Djibouti. Reserve in
seconds, pay at the hotel: no account, no card details, no booking fees.

Built with [Leptos](https://leptos.dev) 0.8 in full-stack SSR mode (Axum server +
WASM hydration) and Tailwind CSS.

## Requirements

- Rust (stable) with the `wasm32-unknown-unknown` target
- [`cargo-leptos`](https://github.com/leptos-rs/cargo-leptos)

```bash
rustup target add wasm32-unknown-unknown
cargo install cargo-leptos
```

## Running locally

```bash
cargo leptos watch
```

Serves on <http://127.0.0.1:3000> and rebuilds the server binary, the WASM
bundle and the stylesheet on change.

## Building for release

```bash
cargo leptos build --release
```

Outputs the server binary to `target/release/` and the site assets to
`target/site/`. Both are needed at runtime; point `LEPTOS_SITE_ROOT` at the
`site` directory.

## Layout

```
src/
  api.rs          Server functions wrapping the Addis Hotel Booking API
  app.rs          Router, shell and the route table
  data.rs         Static marketing copy: testimonials, FAQ, steps, stat labels
  store.rs        Bookings remembered in the visitor's browser
  components/     Header, footer, search widget, hotel cards, gallery, toasts
  pages/          One module per route
style/
  tailwind.css    Design tokens, keyframes and hand-written utilities
```

## Routes

| Path | Page |
| --- | --- |
| `/` | Home |
| `/hotels` | Listings with filters, sort and pagination |
| `/hotels/:id` | Hotel details |
| `/hotels/:id/reserve/:room_id` | Three-step reservation form |
| `/confirmation/:booking_ref` | Booking confirmation |
| `/my-reservations` | Saved reservations |
| `/reservation/:booking_ref` | Reservation details |
| `/retrieve-booking` | Look up a booking by reference |
| `/how-it-works`, `/about`, `/contact`, `/faq` | Content pages |
| `/terms`, `/privacy` | Legal |

## API

Every hotel, room, price, review and booking comes from the Addis Hotel Booking
API at `https://addisapi.pastaromatour.com/api/v1`. Override the base with
`ADDIS_API_BASE` to point a deployment at another backend.

Calls run inside Leptos server functions rather than from the browser, so the
listing and detail pages are server-rendered for search engines and the browser
never talks to a second origin. `src/api.rs` holds the whole client.

What is left in `src/data.rs` is the portal's own editorial — testimonials, the
FAQ, the how-it-works steps — not hotel data. The headline figures are counted
live by `portal_stats()`, so the site never advertises inventory it does not
carry.

### Booking

No accounts. The flow is:

1. `POST /reservations/public/quote/` prices the stay as the guest changes dates
   or party size, and rejects an impossible or already-booked range up front.
2. `POST /reservations/public/book/` creates the reservation against nothing but
   a name and one contact detail.
3. Nothing is charged. The guest pays the hotel on arrival.

### Retrieving a booking

The API reads a booking back only after a one-time code sent to the email or
phone on it (`retrieve/request-otp/`, then `retrieve/`). There is no endpoint
that lists a guest's reservations, so **"My reservations" means the bookings made
or retrieved in this browser** — `src/store.rs` caches each one at the moment we
legitimately hold it. Clearing site data hides the list; the reservations
themselves are always recoverable with a reference and a code.

### Missing upstream

The public hotels endpoint carries no photo, guest score or nightly price. Cards
therefore fall back to the hotel's logo or a monogram, and the "from" price is
summarised in one pass over `/rooms/public/` by `hotel_from_prices()` rather than
fetched per card.

## Status

Complete and live at <https://addis.pastaromatour.com>.
