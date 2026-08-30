//! Client for the Addis Hotel Booking API.
//!
//! Requests run inside Leptos server functions rather than from the browser:
//! the listings and detail pages are server-rendered for SEO, and going through
//! the server also keeps the browser off a second origin (no CORS, no preflight
//! on every navigation).
//!
//! # Wire format
//!
//! Every response is wrapped in an envelope, which the published OpenAPI schema
//! does **not** describe — it documents plain DRF pagination
//! (`{count, next, previous, results}`). The service actually returns:
//!
//! ```json
//! { "success": true, "message": "...", "data": [ … ],
//!   "meta": { "count": 0, "page": 1, "pages": 1, "next": null,
//!             "previous": null, "page_size": 10 } }
//! ```
//!
//! with `meta` omitted on unpaginated collections, and errors as
//! `{"success": false, "code": "...", "message": "...", "errors": {…}}`.
//! The types below follow the observed format, not the schema.

use leptos::prelude::*;
use serde::{Deserialize, Serialize};

/// Override with `ADDIS_API_BASE` to point a deployment at another backend.
pub fn api_base() -> String {
    #[cfg(feature = "ssr")]
    {
        std::env::var("ADDIS_API_BASE")
            .unwrap_or_else(|_| "https://addisapi.pastaromatour.com/api/v1".to_string())
    }
    #[cfg(not(feature = "ssr"))]
    {
        "https://addisapi.pastaromatour.com/api/v1".to_string()
    }
}

// ---------------------------------------------------------------------------
// Envelope
// ---------------------------------------------------------------------------

/// Pagination block. Absent on unpaginated collections, hence the `Default`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Meta {
    #[serde(default)]
    pub count: u32,
    #[serde(default = "one")]
    pub page: u32,
    #[serde(default = "one")]
    pub pages: u32,
    #[serde(default)]
    pub page_size: u32,
}

fn one() -> u32 {
    1
}

/// A page of results plus its pagination metadata.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Page<T> {
    pub items: Vec<T>,
    pub meta: Meta,
}

impl<T> Page<T> {
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

// ---------------------------------------------------------------------------
// DTOs
// ---------------------------------------------------------------------------

/// A hotel as it appears in `GET /organizations/public/`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HotelSummary {
    pub id: i64,
    #[serde(default)]
    pub uuid: String,
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    /// Sent as a decimal string, e.g. `"4.5"`.
    #[serde(default)]
    pub star_rating: Option<String>,
    #[serde(default)]
    pub currency: Option<String>,
    #[serde(default)]
    pub city: Option<String>,
    #[serde(default)]
    pub country: Option<String>,
    #[serde(default)]
    pub address_line: Option<String>,
    #[serde(default)]
    pub logo: Option<String>,
}

impl HotelSummary {
    /// Star rating as a number; `0.0` when absent or unparseable.
    pub fn stars(&self) -> f32 {
        self.star_rating
            .as_deref()
            .and_then(|s| s.parse::<f32>().ok())
            .unwrap_or(0.0)
    }

    /// `"Bole, Addis Ababa, Ethiopia"`, skipping whatever the API left null.
    pub fn location(&self) -> String {
        [
            self.address_line.as_deref(),
            self.city.as_deref(),
            self.country.as_deref(),
        ]
        .into_iter()
        .flatten()
        .filter(|s| !s.trim().is_empty())
        .collect::<Vec<_>>()
        .join(", ")
    }
}

/// One row of `GET /organizations/public/cities/`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct City {
    pub city: String,
    #[serde(default)]
    pub country: Option<String>,
    #[serde(default)]
    pub hotel_count: u32,
    #[serde(default)]
    pub available_rooms_count: u32,
    /// Decimal string, or null when the city has no sellable room.
    #[serde(default)]
    pub min_price: Option<String>,
    #[serde(default)]
    pub featured_image: Option<String>,
}

/// The hotel a room belongs to, as embedded in room payloads.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HotelRef {
    #[serde(default)]
    pub id: i64,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub city: Option<String>,
    #[serde(default)]
    pub country: Option<String>,
    #[serde(default)]
    pub address_line: Option<String>,
    #[serde(default)]
    pub currency: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Amenity {
    #[serde(default)]
    pub id: i64,
    #[serde(default)]
    pub name: String,
    /// `"Essentials"`, `"Services"`, … Absent on the amenity catalogue.
    #[serde(default)]
    pub category: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Bed {
    #[serde(default)]
    pub bed_type: Option<String>,
    #[serde(default)]
    pub number_of_beds: Option<u32>,
}

/// One gallery image on a room or hotel.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Photo {
    #[serde(default)]
    pub id: i64,
    #[serde(default)]
    pub image_url: String,
    #[serde(default)]
    pub caption: Option<String>,
    #[serde(default)]
    pub display_order: i32,
}

/// A stretch of nights a room is off the market, from a room's
/// `unavailable_ranges`. Both dates are inclusive `YYYY-MM-DD`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RoomBlock {
    #[serde(default)]
    pub event_type: Option<String>,
    #[serde(default)]
    pub event_start_date: String,
    #[serde(default)]
    pub event_finished_date: String,
    #[serde(default)]
    pub is_active: bool,
    #[serde(default)]
    pub note: Option<String>,
}

/// A room as it appears in `GET /rooms/public/`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RoomSummary {
    pub id: i64,
    #[serde(default)]
    pub uuid: String,
    #[serde(default)]
    pub room_number: String,
    pub name: String,
    #[serde(default)]
    pub room_type: Option<String>,
    #[serde(default)]
    pub guest_capacity: u32,
    /// Decimal strings on the wire.
    #[serde(default)]
    pub price_per_night: Option<String>,
    #[serde(default)]
    pub price_after_discount: Option<String>,
    #[serde(default)]
    pub discount_percent_per_night: Option<i32>,
    #[serde(default)]
    pub breakfast_included: bool,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub hotel: Option<HotelRef>,
    #[serde(default)]
    pub amenities: Vec<Amenity>,
    #[serde(default)]
    pub beds: Vec<Bed>,
    #[serde(default)]
    pub photos: Vec<Photo>,
    #[serde(default)]
    pub primary_image: Option<String>,
    // ---- detail-only, from `/rooms/public/{id}/` -------------------------
    #[serde(default)]
    pub floor: Option<i32>,
    /// The room's own rules. Shares every field name with the property rules,
    /// so it decodes into the same struct.
    #[serde(default)]
    pub policy: Option<HotelPolicies>,
    /// The property rules, as a fallback for anything the room leaves unset.
    #[serde(default)]
    pub hotel_policy: Option<HotelPolicies>,
    #[serde(default)]
    pub unavailable_ranges: Vec<RoomBlock>,
}

impl RoomSummary {
    /// Price actually charged, in whole currency units.
    pub fn price(&self) -> f64 {
        self.price_after_discount
            .as_deref()
            .or(self.price_per_night.as_deref())
            .and_then(|s| s.parse::<f64>().ok())
            .unwrap_or(0.0)
    }

    /// Pre-discount price, only when it differs from what is charged.
    pub fn was_price(&self) -> Option<f64> {
        let full = self.price_per_night.as_deref()?.parse::<f64>().ok()?;
        (full > self.price()).then_some(full)
    }

    /// `"1 King Bed, 2 Single Beds"`.
    pub fn bed_summary(&self) -> String {
        self.beds
            .iter()
            .filter_map(|b| {
                let t = b.bed_type.as_deref()?;
                Some(match b.number_of_beds.unwrap_or(1) {
                    1 => format!("1 {t}"),
                    n => format!("{n} {t}s"),
                })
            })
            .collect::<Vec<_>>()
            .join(", ")
    }

    /// Total sleeping places, for the "sleeps N" line.
    pub fn bed_count(&self) -> u32 {
        self.beds.iter().map(|b| b.number_of_beds.unwrap_or(1)).sum()
    }

    /// Discount as a whole percentage, `None` when nothing is off.
    pub fn discount(&self) -> Option<i32> {
        self.discount_percent_per_night.filter(|d| *d > 0)
    }

    /// The rules that apply to this room: its own where set, the property's
    /// otherwise.
    pub fn rules(&self) -> HotelPolicies {
        match (self.policy.clone(), self.hotel_policy.clone()) {
            (Some(room), Some(hotel)) => HotelPolicies {
                checkin_time: room.checkin_time.or(hotel.checkin_time),
                checkout_time: room.checkout_time.or(hotel.checkout_time),
                children_allowed: room.children_allowed.or(hotel.children_allowed),
                children_age: room.children_age.or(hotel.children_age),
                extrabed_available: room.extrabed_available.or(hotel.extrabed_available),
                pet_allowed: room.pet_allowed.or(hotel.pet_allowed),
                smoking_allowed: room.smoking_allowed.or(hotel.smoking_allowed),
                non_smoking_property: room.non_smoking_property.or(hotel.non_smoking_property),
                government_id_required: room
                    .government_id_required
                    .or(hotel.government_id_required),
                minimum_checkin_age: room.minimum_checkin_age.or(hotel.minimum_checkin_age),
                parties_or_event_allowed: room
                    .parties_or_event_allowed
                    .or(hotel.parties_or_event_allowed),
                bank_card_allow: hotel.bank_card_allow,
                online_transaction_allow: hotel.online_transaction_allow,
                bank_payment_allow: hotel.bank_payment_allow,
                public_note: room.public_note.or(hotel.public_note),
            },
            (Some(room), None) => room,
            (None, Some(hotel)) => hotel,
            (None, None) => HotelPolicies::default(),
        }
    }

    /// Amenities grouped by their category, categories in first-seen order so
    /// the API's own ordering carries through.
    pub fn amenities_by_category(&self) -> Vec<(String, Vec<String>)> {
        let mut out: Vec<(String, Vec<String>)> = Vec::new();
        for a in &self.amenities {
            let key = a
                .category
                .clone()
                .filter(|c| !c.trim().is_empty())
                .unwrap_or_else(|| "Amenities".into());
            match out.iter_mut().find(|(k, _)| *k == key) {
                Some((_, list)) => list.push(a.name.clone()),
                None => out.push((key, vec![a.name.clone()])),
            }
        }
        out
    }

    /// Upcoming stretches the room cannot be booked for.
    pub fn blocks(&self) -> Vec<&RoomBlock> {
        self.unavailable_ranges
            .iter()
            .filter(|b| b.is_active && !b.event_start_date.is_empty())
            .collect()
    }

    /// `true` when the room is sellable right now.
    pub fn is_bookable(&self) -> bool {
        self.status
            .as_deref()
            .map(|s| s.eq_ignore_ascii_case("Available"))
            .unwrap_or(true)
    }
}

// ---------------------------------------------------------------------------
// Query parameters
// ---------------------------------------------------------------------------

/// Filters for `GET /organizations/public/`. Empty fields are left off the URL
/// so the API applies its own defaults.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct HotelQuery {
    pub search: Option<String>,
    pub city: Option<String>,
    pub country: Option<String>,
    pub min_rating: Option<f32>,
    pub max_rating: Option<f32>,
    /// Comma-separated amenity ids, as the API expects.
    pub amenities: Option<String>,
    pub check_in: Option<String>,
    pub check_out: Option<String>,
    pub ordering: Option<String>,
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}

/// Filters for `GET /rooms/public/`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct RoomQuery {
    pub organization_id: Option<String>,
    pub search: Option<String>,
    pub city: Option<String>,
    pub room_type: Option<String>,
    pub guest_capacity: Option<u32>,
    pub min_price: Option<f64>,
    pub max_price: Option<f64>,
    pub breakfast_included: Option<bool>,
    pub check_in: Option<String>,
    pub check_out: Option<String>,
    pub ordering: Option<String>,
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}

// ---------------------------------------------------------------------------
// Transport (server side only)
// ---------------------------------------------------------------------------

#[cfg(feature = "ssr")]
mod transport {
    use super::{api_base, Meta, Page};
    use leptos::prelude::ServerFnError;
    use serde::de::DeserializeOwned;

    /// Query-string pairs, skipping anything the caller left unset.
    pub type Params = Vec<(&'static str, String)>;

    fn client() -> Result<reqwest::Client, ServerFnError> {
        reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(20))
            .user_agent("addis-landing/1.0")
            .build()
            .map_err(|e| ServerFnError::new(format!("http client: {e}")))
    }

    /// Pulls the human-readable message out of an error envelope so the page can
    /// show what the API actually objected to instead of a bare status code.
    fn api_error(_status: reqwest::StatusCode, body: &str) -> ServerFnError {
        let parsed = serde_json::from_str::<serde_json::Value>(body).ok();
        let msg = parsed
            .as_ref()
            .and_then(|v| v.get("message").and_then(|m| m.as_str()).map(str::to_owned))
            .unwrap_or_else(|| body.chars().take(200).collect());
        // A validation failure names the offending field; showing that beats a
        // generic "check your details".
        let field = parsed
            .as_ref()
            .and_then(|v| v.get("errors"))
            .and_then(|e| e.as_object())
            .and_then(|obj| {
                obj.iter().find_map(|(k, val)| {
                    let text = match val {
                        serde_json::Value::Array(a) => {
                            a.iter().filter_map(|x| x.as_str()).collect::<Vec<_>>().join(" ")
                        }
                        serde_json::Value::String(s) => s.clone(),
                        _ => return None,
                    };
                    (!text.is_empty()).then(|| {
                        if k == "non_field_errors" { text } else { format!("{k}: {text}") }
                    })
                })
            });
        ServerFnError::new(field.unwrap_or(msg))
    }

    async fn get_json(path: &str, params: &Params) -> Result<serde_json::Value, ServerFnError> {
        let res = client()?
            .get(format!("{}{path}", api_base()))
            .query(params)
            .send()
            .await
            .map_err(|e| ServerFnError::new(format!("request to {path} failed: {e}")))?;

        let status = res.status();
        let body = res
            .text()
            .await
            .map_err(|e| ServerFnError::new(format!("reading {path} failed: {e}")))?;

        if !status.is_success() {
            return Err(api_error(status, &body));
        }
        serde_json::from_str(&body)
            .map_err(|e| ServerFnError::new(format!("decoding {path} failed: {e}")))
    }

    /// Fetches a paginated collection and unwraps `{data, meta}`.
    pub async fn get_page<T: DeserializeOwned>(
        path: &str,
        params: &Params,
    ) -> Result<Page<T>, ServerFnError> {
        let v = get_json(path, params).await?;
        let items = serde_json::from_value::<Vec<T>>(
            v.get("data").cloned().unwrap_or(serde_json::Value::Null),
        )
        .map_err(|e| ServerFnError::new(format!("decoding {path} rows failed: {e}")))?;
        let meta = v
            .get("meta")
            .and_then(|m| serde_json::from_value::<Meta>(m.clone()).ok())
            .unwrap_or(Meta {
                count: items.len() as u32,
                page: 1,
                pages: 1,
                page_size: items.len() as u32,
            });
        Ok(Page { items, meta })
    }

    /// Fetches a single object and unwraps `{data}`.
    pub async fn get_one<T: DeserializeOwned>(
        path: &str,
        params: &Params,
    ) -> Result<T, ServerFnError> {
        let v = get_json(path, params).await?;
        serde_json::from_value::<T>(v.get("data").cloned().unwrap_or(v))
            .map_err(|e| ServerFnError::new(format!("decoding {path} failed: {e}")))
    }

    /// Fetches a bare array (`data` is a list with no `meta`).
    pub async fn get_list<T: DeserializeOwned>(
        path: &str,
        params: &Params,
    ) -> Result<Vec<T>, ServerFnError> {
        let v = get_json(path, params).await?;
        let data = v.get("data").cloned().unwrap_or(serde_json::Value::Array(vec![]));
        if data.is_null() {
            return Ok(Vec::new());
        }
        serde_json::from_value::<Vec<T>>(data)
            .map_err(|e| ServerFnError::new(format!("decoding {path} failed: {e}")))
    }

    /// POSTs JSON and unwraps `{data}`.
    ///
    /// Validation failures come back as a 400 carrying the reason in `message`
    /// (and sometimes per-field detail in `errors`), so the error is flattened
    /// into something the form can show the guest directly.
    pub async fn post_json<T: DeserializeOwned>(
        path: &str,
        body: &serde_json::Value,
    ) -> Result<T, ServerFnError> {
        let res = client()?
            .post(format!("{}{path}", api_base()))
            .json(body)
            .send()
            .await
            .map_err(|e| ServerFnError::new(format!("request to {path} failed: {e}")))?;

        let status = res.status();
        let text = res
            .text()
            .await
            .map_err(|e| ServerFnError::new(format!("reading {path} failed: {e}")))?;

        if !status.is_success() {
            return Err(api_error(status, &text));
        }
        let v: serde_json::Value = serde_json::from_str(&text)
            .map_err(|e| ServerFnError::new(format!("decoding {path} failed: {e}")))?;
        serde_json::from_value::<T>(v.get("data").cloned().unwrap_or(v))
            .map_err(|e| ServerFnError::new(format!("decoding {path} failed: {e}")))
    }
}

// ---------------------------------------------------------------------------
// Server functions
// ---------------------------------------------------------------------------

/// Browse hotels. Backs `/hotels` and the home page's featured rail.
#[server(name = ListHotels, prefix = "/api")]
pub async fn list_hotels(query: HotelQuery) -> Result<Page<HotelSummary>, ServerFnError> {
    let mut p: transport::Params = Vec::new();
    let mut push = |k: &'static str, v: Option<String>| {
        if let Some(v) = v.filter(|v| !v.trim().is_empty()) {
            p.push((k, v));
        }
    };
    push("search", query.search);
    push("city", query.city);
    push("country", query.country);
    push("amenities", query.amenities);
    push("check_in", query.check_in);
    push("check_out", query.check_out);
    push("ordering", query.ordering);
    push("min_rating", query.min_rating.map(|v| v.to_string()));
    push("max_rating", query.max_rating.map(|v| v.to_string()));
    push("page", query.page.map(|v| v.to_string()));
    push("page_size", query.page_size.map(|v| v.to_string()));

    transport::get_page("/organizations/public/", &p).await
}

/// One hotel, for `/hotels/:id`.
#[server(name = GetHotel, prefix = "/api")]
pub async fn get_hotel(id: String) -> Result<HotelSummary, ServerFnError> {
    transport::get_one(&format!("/organizations/public/{id}/"), &Vec::new()).await
}

/// Cities with hotel counts and a from-price. Backs the home page rail.
#[server(name = ListCities, prefix = "/api")]
pub async fn list_cities(page_size: Option<u32>) -> Result<Page<City>, ServerFnError> {
    let mut p: transport::Params = Vec::new();
    if let Some(n) = page_size {
        p.push(("page_size", n.to_string()));
    }
    transport::get_page("/organizations/public/cities/", &p).await
}

/// Bookable rooms, optionally scoped to one hotel.
#[server(name = ListRooms, prefix = "/api")]
pub async fn list_rooms(query: RoomQuery) -> Result<Page<RoomSummary>, ServerFnError> {
    let mut p: transport::Params = Vec::new();
    let mut push = |k: &'static str, v: Option<String>| {
        if let Some(v) = v.filter(|v| !v.trim().is_empty()) {
            p.push((k, v));
        }
    };
    push("organization_id", query.organization_id);
    push("search", query.search);
    push("city", query.city);
    push("room_type", query.room_type);
    push("check_in", query.check_in);
    push("check_out", query.check_out);
    push("ordering", query.ordering);
    push("guest_capacity", query.guest_capacity.map(|v| v.to_string()));
    push("min_price", query.min_price.map(|v| v.to_string()));
    push("max_price", query.max_price.map(|v| v.to_string()));
    push(
        "breakfast_included",
        query.breakfast_included.map(|v| v.to_string()),
    );
    push("page", query.page.map(|v| v.to_string()));
    push("page_size", query.page_size.map(|v| v.to_string()));

    transport::get_page("/rooms/public/", &p).await
}

/// The amenity catalogue, used to turn the filter rail's checkboxes into the
/// comma-separated ids `/organizations/public/` expects.
#[server(name = ListAmenities, prefix = "/api")]
pub async fn list_amenities() -> Result<Vec<Amenity>, ServerFnError> {
    Ok(transport::get_page::<Amenity>("/amenities/", &Vec::new())
        .await?
        .items)
}

/// One room, for the reservation flow.
#[server(name = GetRoom, prefix = "/api")]
pub async fn get_room(id: String) -> Result<RoomSummary, ServerFnError> {
    transport::get_one(&format!("/rooms/public/{id}/"), &Vec::new()).await
}

// ---------------------------------------------------------------------------
// Hotel detail
// ---------------------------------------------------------------------------

/// The property rules shown on a hotel page.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct HotelPolicies {
    #[serde(default)]
    pub checkin_time: Option<String>,
    #[serde(default)]
    pub checkout_time: Option<String>,
    #[serde(default)]
    pub children_allowed: Option<bool>,
    #[serde(default)]
    pub children_age: Option<i32>,
    #[serde(default)]
    pub extrabed_available: Option<bool>,
    #[serde(default)]
    pub pet_allowed: Option<bool>,
    #[serde(default)]
    pub smoking_allowed: Option<bool>,
    #[serde(default)]
    pub non_smoking_property: Option<bool>,
    #[serde(default)]
    pub government_id_required: Option<bool>,
    #[serde(default)]
    pub minimum_checkin_age: Option<i32>,
    #[serde(default)]
    pub parties_or_event_allowed: Option<bool>,
    #[serde(default)]
    pub bank_card_allow: Option<bool>,
    #[serde(default)]
    pub online_transaction_allow: Option<bool>,
    #[serde(default)]
    pub bank_payment_allow: Option<bool>,
    #[serde(default)]
    pub public_note: Option<String>,
}

impl HotelPolicies {
    /// `"14:00:00"` -> `"2:00 PM"`. Blank input yields an em dash.
    pub fn checkin_display(&self) -> String {
        pretty_time(self.checkin_time.as_deref())
    }
    pub fn checkout_display(&self) -> String {
        pretty_time(self.checkout_time.as_deref())
    }

    /// House rules worth listing, as `(icon, text)` pairs. Only rules the
    /// property actually set are returned — an unanswered `null` is not a "no".
    pub fn rules(&self) -> Vec<(&'static str, String)> {
        let mut out = Vec::new();
        if let Some(allowed) = self.children_allowed {
            out.push((
                "family",
                match (allowed, self.children_age) {
                    (true, Some(age)) => format!("Children welcome, under {age} stay free"),
                    (true, None) => "Children are welcome".to_string(),
                    (false, _) => "Adults only".to_string(),
                },
            ));
        }
        if let Some(extra) = self.extrabed_available {
            out.push((
                "bed",
                if extra { "Extra beds available on request".into() } else { "No extra beds".into() },
            ));
        }
        if let Some(pets) = self.pet_allowed {
            out.push((
                "paw",
                if pets { "Pets allowed".into() } else { "Pets are not allowed".into() },
            ));
        }
        if self.non_smoking_property == Some(true) {
            out.push(("shirt-off", "Entirely non-smoking property".into()));
        } else if let Some(smoking) = self.smoking_allowed {
            out.push((
                "shirt-off",
                if smoking { "Smoking permitted in designated areas".into() } else { "No smoking".into() },
            ));
        }
        if let Some(parties) = self.parties_or_event_allowed {
            out.push((
                "wine",
                if parties { "Parties and events allowed".into() } else { "No parties or events".into() },
            ));
        }
        if self.government_id_required == Some(true) {
            out.push(("id-card", "Government ID required at check-in".into()));
        }
        if let Some(age) = self.minimum_checkin_age {
            out.push(("user-check", format!("Minimum check-in age is {age}")));
        }
        out
    }

    /// Payment methods the property accepts on arrival.
    pub fn payment_methods(&self) -> Vec<&'static str> {
        let mut out = vec!["Cash"];
        if self.bank_card_allow == Some(true) {
            out.push("Bank card");
        }
        if self.bank_payment_allow == Some(true) {
            out.push("Bank transfer");
        }
        if self.online_transaction_allow == Some(true) {
            out.push("Online payment");
        }
        out
    }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct HotelContact {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub email: Option<String>,
    #[serde(default)]
    pub phone_number: Option<String>,
    #[serde(default)]
    pub is_primary: bool,
}

/// A hotel as it appears on `GET /organizations/public/{id}/`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct HotelDetail {
    pub id: i64,
    #[serde(default)]
    pub uuid: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub star_rating: Option<String>,
    #[serde(default)]
    pub currency: Option<String>,
    #[serde(default)]
    pub established_year: Option<i32>,
    #[serde(default)]
    pub address_line: Option<String>,
    #[serde(default)]
    pub latitude: Option<f64>,
    #[serde(default)]
    pub longitude: Option<f64>,
    #[serde(default)]
    pub city: Option<String>,
    #[serde(default)]
    pub postal_code: Option<String>,
    #[serde(default)]
    pub country: Option<String>,
    #[serde(default)]
    pub email: Option<String>,
    #[serde(default)]
    pub phone_number: Option<String>,
    #[serde(default)]
    pub whatsapp: Option<String>,
    #[serde(default)]
    pub website_url: Option<String>,
    #[serde(default)]
    pub fb_link: Option<String>,
    #[serde(default)]
    pub instagram_link: Option<String>,
    #[serde(default)]
    pub photos: Vec<Photo>,
    #[serde(default)]
    pub policies: Option<HotelPolicies>,
    #[serde(default)]
    pub amenities: Vec<Amenity>,
    #[serde(default)]
    pub contacts: Vec<HotelContact>,
}

impl HotelDetail {
    pub fn stars(&self) -> f32 {
        self.star_rating
            .as_deref()
            .and_then(|s| s.parse().ok())
            .unwrap_or(0.0)
    }
    pub fn currency_code(&self) -> &str {
        self.currency.as_deref().unwrap_or("ETB")
    }
    pub fn location(&self) -> String {
        [
            self.address_line.as_deref(),
            self.city.as_deref(),
            self.country.as_deref(),
        ]
        .into_iter()
        .flatten()
        .filter(|s| !s.trim().is_empty())
        .collect::<Vec<_>>()
        .join(", ")
    }
    pub fn city_line(&self) -> String {
        [self.city.as_deref(), self.country.as_deref()]
            .into_iter()
            .flatten()
            .filter(|s| !s.trim().is_empty())
            .collect::<Vec<_>>()
            .join(", ")
    }
    /// Gallery images, ordered, falling back to whatever the rooms provide.
    pub fn gallery(&self) -> Vec<String> {
        let mut photos = self.photos.clone();
        photos.sort_by_key(|p| p.display_order);
        photos
            .into_iter()
            .map(|p| p.image_url)
            .filter(|u| u.starts_with("http"))
            .collect()
    }
    pub fn initial(&self) -> String {
        self.name
            .chars()
            .next()
            .map(|c| c.to_uppercase().to_string())
            .unwrap_or_else(|| "?".into())
    }
}

/// `"14:00:00"` -> `"2:00 PM"`.
pub fn pretty_time(raw: Option<&str>) -> String {
    let Some(t) = raw.map(str::trim).filter(|s| !s.is_empty()) else {
        return "—".into();
    };
    let mut parts = t.split(':');
    let (Some(h), Some(m)) = (parts.next(), parts.next()) else {
        return t.to_string();
    };
    let Ok(hour) = h.parse::<u32>() else {
        return t.to_string();
    };
    let suffix = if hour < 12 { "AM" } else { "PM" };
    let display = match hour % 12 {
        0 => 12,
        other => other,
    };
    format!("{display}:{m} {suffix}")
}

/// `"2026-08-28"` -> `"28 Aug 2026"`.
pub fn pretty_date(raw: Option<&str>) -> String {
    let Some(s) = raw.map(str::trim).filter(|s| !s.is_empty()) else {
        return "—".into();
    };
    let date = s.split('T').next().unwrap_or(s);
    let parts: Vec<&str> = date.split('-').collect();
    if parts.len() != 3 {
        return s.to_string();
    }
    const MONTHS: [&str; 12] = [
        "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ];
    match (parts[1].parse::<usize>(), parts[2].parse::<u32>()) {
        (Ok(m), Ok(d)) if (1..=12).contains(&m) => format!("{d} {} {}", MONTHS[m - 1], parts[0]),
        _ => s.to_string(),
    }
}

/// `12345.6` -> `"12,345.60"`.
pub fn money(n: f64) -> String {
    let whole = n.trunc().abs() as i64;
    let cents = (n.abs().fract() * 100.0).round() as i64;
    let s = whole.to_string();
    let mut grouped = String::new();
    for (i, c) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            grouped.push(',');
        }
        grouped.push(c);
    }
    let grouped: String = grouped.chars().rev().collect();
    let sign = if n < 0.0 { "-" } else { "" };
    format!("{sign}{grouped}.{:02}", cents.min(99))
}

/// Whole units, for prices where cents are noise.
pub fn money_round(n: f64) -> String {
    let s = (n.round().abs() as i64).to_string();
    let mut grouped = String::new();
    for (i, c) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            grouped.push(',');
        }
        grouped.push(c);
    }
    let grouped: String = grouped.chars().rev().collect();
    if n < 0.0 {
        format!("-{grouped}")
    } else {
        grouped
    }
}

// ---------------------------------------------------------------------------
// Booking
// ---------------------------------------------------------------------------

/// The priced breakdown `POST /reservations/public/quote/` returns.
///
/// Every field is a number here, unlike the reservation endpoints which send
/// decimals as strings.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Quote {
    #[serde(default)]
    pub room_id: i64,
    #[serde(default)]
    pub room_name: String,
    #[serde(default)]
    pub room_number: String,
    #[serde(default)]
    pub check_in_date: String,
    #[serde(default)]
    pub check_out_date: String,
    #[serde(default)]
    pub number_of_nights: u32,
    #[serde(default)]
    pub daily_room_rate: f64,
    #[serde(default)]
    pub base_room_charge: f64,
    #[serde(default)]
    pub extra_bed_count: u32,
    #[serde(default)]
    pub extra_bed_per_night_price: f64,
    #[serde(default)]
    pub extra_bed_charge: f64,
    #[serde(default)]
    pub pet_presence: bool,
    #[serde(default)]
    pub pet_charge: f64,
    #[serde(default)]
    pub subtotal: f64,
    #[serde(default)]
    pub discount_amount: f64,
    #[serde(default)]
    pub taxable_amount: f64,
    #[serde(default)]
    pub tax_rate_percent: f64,
    #[serde(default)]
    pub tax_amount: f64,
    #[serde(default)]
    pub total_amount: f64,
}

/// What the reservation form collects.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct BookingRequest {
    pub organization_id: i64,
    pub room_id: i64,
    pub check_in_date: String,
    pub check_out_date: String,
    pub guest_count: u32,
    pub extra_bed: u32,
    pub pet_presence: bool,
    pub baby_presence: bool,
    pub guest_first_name: String,
    pub guest_last_name: String,
    pub guest_email: String,
    pub guest_phone: String,
    pub guest_note: String,
}

/// The room, as embedded in a public reservation.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct BookedRoom {
    #[serde(default)]
    pub id: i64,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub room_number: String,
    #[serde(default)]
    pub room_type: Option<String>,
    #[serde(default)]
    pub price_per_night: Option<String>,
    #[serde(default)]
    pub price_after_discount: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct BookedGuest {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub email: Option<String>,
    #[serde(default)]
    pub phone_number: Option<String>,
}

/// A booking as the guest sees it — from `book/` or `retrieve/`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Reservation {
    #[serde(default)]
    pub id: i64,
    #[serde(default)]
    pub uuid: String,
    #[serde(default)]
    pub booking_reference: String,
    #[serde(default)]
    pub reference_display: Option<String>,
    #[serde(default)]
    pub organization: i64,
    #[serde(default)]
    pub organization_name: String,
    #[serde(default)]
    pub room: BookedRoom,
    #[serde(default)]
    pub guest: BookedGuest,
    #[serde(default)]
    pub check_in_date: Option<String>,
    #[serde(default)]
    pub check_out_date: Option<String>,
    #[serde(default)]
    pub number_of_nights: u32,
    #[serde(default)]
    pub guest_count: u32,
    #[serde(default)]
    pub extra_bed: u32,
    #[serde(default)]
    pub pet_presence: bool,
    #[serde(default)]
    pub baby_presence: bool,
    #[serde(default)]
    pub booking_status: String,
    #[serde(default)]
    pub cancellation_reason: Option<String>,
    #[serde(default)]
    pub guest_note: Option<String>,
    #[serde(default)]
    pub created_at: Option<String>,
}

impl Reservation {
    pub fn reference(&self) -> String {
        self.reference_display
            .clone()
            .filter(|r| !r.is_empty())
            .unwrap_or_else(|| self.booking_reference.clone())
    }
    pub fn status(&self) -> &str {
        if self.booking_status.is_empty() {
            "New"
        } else {
            &self.booking_status
        }
    }
    /// What the stay is worth at the room's published rate. The public
    /// endpoints do not return a total, so this is the same arithmetic the
    /// quote endpoint does before tax.
    pub fn room_subtotal(&self) -> f64 {
        let rate = self
            .room
            .price_after_discount
            .as_deref()
            .or(self.room.price_per_night.as_deref())
            .and_then(|s| s.parse::<f64>().ok())
            .unwrap_or(0.0);
        rate * self.number_of_nights as f64
    }
    pub fn stay_dates(&self) -> String {
        format!(
            "{} → {}",
            pretty_date(self.check_in_date.as_deref()),
            pretty_date(self.check_out_date.as_deref())
        )
    }
    /// Whether the booking is still live, as opposed to cancelled or rejected.
    pub fn is_active(&self) -> bool {
        !matches!(self.status(), "Cancelled" | "Rejected" | "No show")
    }
}

/// A date range this room is already taken or blocked for.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct BlockedRange {
    pub start: String,
    pub end: String,
}

// ---------------------------------------------------------------------------
// Server functions — hotel & room detail
// ---------------------------------------------------------------------------

/// Full hotel record for `/hotels/:id`, including policies and contacts.
#[server(name = GetHotelDetail, prefix = "/api")]
pub async fn get_hotel_detail(id: String) -> Result<HotelDetail, ServerFnError> {
    transport::get_one(&format!("/organizations/public/{id}/"), &Vec::new()).await
}

/// Bookable rooms at one hotel, cheapest first.
#[server(name = ListHotelRooms, prefix = "/api")]
pub async fn list_hotel_rooms(
    organization_id: String,
    check_in: Option<String>,
    check_out: Option<String>,
) -> Result<Vec<RoomSummary>, ServerFnError> {
    let mut p: transport::Params = vec![
        ("organization_id", organization_id),
        ("page_size", "100".to_string()),
        ("ordering", "price_per_night".to_string()),
    ];
    if let Some(v) = check_in.filter(|v| !v.trim().is_empty()) {
        p.push(("check_in", v));
    }
    if let Some(v) = check_out.filter(|v| !v.trim().is_empty()) {
        p.push(("check_out", v));
    }
    Ok(transport::get_page::<RoomSummary>("/rooms/public/", &p)
        .await?
        .items)
}

/// The cheapest nightly rate per hotel, for the "from" price on listing cards.
///
/// `GET /organizations/public/` carries no price, so one pass over the room feed
/// supplies it for every hotel on the page at once.
#[server(name = HotelFromPrices, prefix = "/api")]
pub async fn hotel_from_prices() -> Result<Vec<(i64, f64)>, ServerFnError> {
    let rooms = transport::get_page::<RoomSummary>(
        "/rooms/public/",
        &vec![("page_size", "100".to_string())],
    )
    .await?
    .items;

    let mut out: Vec<(i64, f64)> = Vec::new();
    for r in rooms {
        let Some(hotel_id) = r.hotel.as_ref().map(|h| h.id) else {
            continue;
        };
        let price = r.price();
        if price <= 0.0 {
            continue;
        }
        match out.iter_mut().find(|(id, _)| *id == hotel_id) {
            Some(entry) => entry.1 = entry.1.min(price),
            None => out.push((hotel_id, price)),
        }
    }
    Ok(out)
}

// ---------------------------------------------------------------------------
// Server functions — booking
// ---------------------------------------------------------------------------

/// Prices a stay before the guest commits. Also the validity check: the API
/// rejects impossible ranges here rather than at booking time.
#[server(name = GetQuote, prefix = "/api")]
pub async fn get_quote(
    room_id: i64,
    check_in_date: String,
    check_out_date: String,
    guest_count: u32,
    extra_bed: u32,
    pet_presence: bool,
) -> Result<Quote, ServerFnError> {
    let body = serde_json::json!({
        "room_id": room_id,
        "check_in_date": check_in_date,
        "check_out_date": check_out_date,
        "guest_count": guest_count,
        "extra_bed": extra_bed,
        "pet_presence": pet_presence,
    });
    transport::post_json("/reservations/public/quote/", &body).await
}

/// Creates the booking. No account is required — the guest is identified by the
/// email or phone they give here, which is also how they retrieve it later.
#[server(name = CreateBooking, prefix = "/api")]
pub async fn create_booking(req: BookingRequest) -> Result<Reservation, ServerFnError> {
    let body = serde_json::to_value(&req)
        .map_err(|e| ServerFnError::new(format!("could not encode the booking: {e}")))?;
    transport::post_json("/reservations/public/book/", &body).await
}

/// Dates a room is already taken, for the date picker.
#[server(name = RoomBlockedDates, prefix = "/api")]
pub async fn room_blocked_dates(
    room_id: i64,
    year: i32,
    month: u32,
) -> Result<Vec<BlockedRange>, ServerFnError> {
    let p: transport::Params = vec![
        ("room_id", room_id.to_string()),
        ("year", year.to_string()),
        ("month", month.to_string()),
    ];
    transport::get_list("/reservations/public/availability/", &p).await
}

// ---------------------------------------------------------------------------
// Server functions — retrieving a booking
// ---------------------------------------------------------------------------

/// Sends a six-digit code to the email or phone on the booking.
#[server(name = RequestBookingOtp, prefix = "/api")]
pub async fn request_booking_otp(
    booking_reference: String,
    guest_email: String,
    guest_phone: String,
) -> Result<String, ServerFnError> {
    let mut body = serde_json::Map::new();
    body.insert(
        "booking_reference".into(),
        serde_json::json!(booking_reference.trim()),
    );
    if !guest_email.trim().is_empty() {
        body.insert("guest_email".into(), serde_json::json!(guest_email.trim()));
    }
    if !guest_phone.trim().is_empty() {
        body.insert("guest_phone".into(), serde_json::json!(guest_phone.trim()));
    }
    // The endpoint's `data` is not a fixed shape; the caller only needs to know
    // it succeeded.
    let _: serde_json::Value = transport::post_json(
        "/reservations/public/retrieve/request-otp/",
        &serde_json::Value::Object(body),
    )
    .await?;
    Ok("sent".to_string())
}

/// Exchanges the code for the booking.
#[server(name = RetrieveBooking, prefix = "/api")]
pub async fn retrieve_booking(
    booking_reference: String,
    guest_email: String,
    guest_phone: String,
    otp_code: String,
) -> Result<Reservation, ServerFnError> {
    let mut body = serde_json::Map::new();
    body.insert(
        "booking_reference".into(),
        serde_json::json!(booking_reference.trim()),
    );
    body.insert("otp_code".into(), serde_json::json!(otp_code.trim()));
    if !guest_email.trim().is_empty() {
        body.insert("guest_email".into(), serde_json::json!(guest_email.trim()));
    }
    if !guest_phone.trim().is_empty() {
        body.insert("guest_phone".into(), serde_json::json!(guest_phone.trim()));
    }
    transport::post_json(
        "/reservations/public/retrieve/",
        &serde_json::Value::Object(body),
    )
    .await
}

// ---------------------------------------------------------------------------
// Public reviews
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ReviewBooking {
    #[serde(default)]
    pub room_number: Option<String>,
    #[serde(default)]
    pub room_type: Option<String>,
    #[serde(default)]
    pub check_in_date: Option<String>,
    #[serde(default)]
    pub check_out_date: Option<String>,
}

/// A guest review, as shown on a hotel page.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Review {
    #[serde(default)]
    pub id: i64,
    #[serde(default)]
    pub rate: i32,
    #[serde(default)]
    pub comment: String,
    #[serde(default)]
    pub reply: Option<String>,
    #[serde(default)]
    pub replied_at: Option<String>,
    #[serde(default)]
    pub replied_by_name: Option<String>,
    #[serde(default)]
    pub guest_name: String,
    #[serde(default)]
    pub booking_info: Option<ReviewBooking>,
    #[serde(default)]
    pub created_at: Option<String>,
}

impl Review {
    pub fn initials(&self) -> String {
        self.guest_name
            .split_whitespace()
            .filter_map(|w| w.chars().next())
            .take(2)
            .collect::<String>()
            .to_uppercase()
    }
    pub fn has_reply(&self) -> bool {
        self.reply.as_deref().is_some_and(|r| !r.trim().is_empty())
    }
    pub fn stayed_in(&self) -> Option<String> {
        let b = self.booking_info.as_ref()?;
        let room = b.room_type.clone().or(b.room_number.clone())?;
        Some(match b.check_in_date.as_deref() {
            Some(d) => format!("Stayed in {room} · {}", pretty_date(Some(d))),
            None => format!("Stayed in {room}"),
        })
    }
}

/// Reviews plus the aggregate the page header shows.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct HotelReviews {
    pub items: Vec<Review>,
    pub count: u32,
    pub average: f32,
    /// Five-star first, matching how the histogram is drawn.
    pub breakdown: [u32; 5],
}

#[server(name = ListHotelReviews, prefix = "/api")]
pub async fn list_hotel_reviews(organization_id: String) -> Result<HotelReviews, ServerFnError> {
    let page = transport::get_page::<Review>(
        "/reviews/",
        &vec![
            ("organization_id", organization_id),
            ("page_size", "100".to_string()),
            ("ordering", "-created_at".to_string()),
        ],
    )
    .await?;

    let items = page.items;
    let count = items.len() as u32;
    let average = if count == 0 {
        0.0
    } else {
        items.iter().map(|r| r.rate as f32).sum::<f32>() / count as f32
    };
    let mut breakdown = [0u32; 5];
    for r in &items {
        if (1..=5).contains(&r.rate) {
            // Index 0 is five stars.
            breakdown[(5 - r.rate) as usize] += 1;
        }
    }
    Ok(HotelReviews {
        items,
        count,
        average,
        breakdown,
    })
}

// ---------------------------------------------------------------------------
// Portal-wide figures
// ---------------------------------------------------------------------------

/// Headline numbers for the home page.
///
/// These used to be invented marketing figures. They are counted from the API
/// instead, so the page never claims more inventory than the portal actually
/// carries.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct PortalStats {
    pub hotels: u32,
    pub cities: u32,
    pub countries: u32,
    pub rooms: u32,
    pub reviews: u32,
    /// Mean guest rating, or the mean star class when nobody has reviewed yet.
    pub average_rating: f32,
}

impl PortalStats {
    /// `"690+"`-style copy is misleading at small numbers, so exact counts are
    /// shown until there are enough to round.
    pub fn hotels_display(&self) -> String {
        if self.hotels >= 50 {
            format!("{}+", self.hotels / 10 * 10)
        } else {
            self.hotels.to_string()
        }
    }
    pub fn rooms_display(&self) -> String {
        if self.rooms >= 50 {
            format!("{}+", self.rooms / 10 * 10)
        } else {
            self.rooms.to_string()
        }
    }
    /// `"2 verified hotels · 1 country"` for the hero badge.
    pub fn badge(&self) -> String {
        let hotels = if self.hotels == 1 { "hotel" } else { "hotels" };
        let countries = if self.countries == 1 { "country" } else { "countries" };
        format!(
            "{} verified {hotels} · {} {countries}",
            self.hotels_display(),
            self.countries,
        )
    }
}

#[server(name = GetPortalStats, prefix = "/api")]
pub async fn portal_stats() -> Result<PortalStats, ServerFnError> {
    let hotels = transport::get_page::<HotelSummary>(
        "/organizations/public/",
        &vec![("page_size", "100".to_string())],
    )
    .await?;

    let cities = transport::get_page::<City>(
        "/organizations/public/cities/",
        &vec![("page_size", "100".to_string())],
    )
    .await
    .map(|p| p.items)
    .unwrap_or_default();

    let rooms = transport::get_page::<RoomSummary>(
        "/rooms/public/",
        &vec![("page_size", "1".to_string())],
    )
    .await
    .map(|p| p.meta.count)
    .unwrap_or(0);

    let reviews = transport::get_page::<Review>(
        "/reviews/",
        &vec![("page_size", "100".to_string())],
    )
    .await
    .map(|p| p.items)
    .unwrap_or_default();

    let mut countries: Vec<String> = cities
        .iter()
        .filter_map(|c| c.country.clone())
        .filter(|c| !c.trim().is_empty())
        .collect();
    countries.sort();
    countries.dedup();

    // Guest ratings when there are any, otherwise the properties' star classes —
    // never a made-up number.
    let average_rating = if !reviews.is_empty() {
        reviews.iter().map(|r| r.rate as f32).sum::<f32>() / reviews.len() as f32
    } else {
        let rated: Vec<f32> = hotels
            .items
            .iter()
            .map(|h| h.stars())
            .filter(|s| *s > 0.0)
            .collect();
        if rated.is_empty() {
            0.0
        } else {
            rated.iter().sum::<f32>() / rated.len() as f32
        }
    };

    Ok(PortalStats {
        hotels: hotels.meta.count.max(hotels.items.len() as u32),
        cities: cities.len() as u32,
        countries: countries.len() as u32,
        rooms,
        reviews: reviews.len() as u32,
        average_rating,
    })
}
