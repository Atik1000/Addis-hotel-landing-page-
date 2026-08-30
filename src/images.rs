//! Curated fallback imagery.
//!
//! The API stores no artwork for hotels or cities: `logo` on
//! `/organizations/public/` and `featured_image` on
//! `/organizations/public/cities/` both come back as empty strings for every
//! record, and only half the rooms carry a `primary_image`. Rendered straight
//! from the feed the site therefore shows letter tiles and flat gradients
//! rather than photographs.
//!
//! These pools stand in until a hotel uploads its own pictures through the CRM.
//! A record's own image always wins — see [`hotel_cover`] and [`room_image`] —
//! and the fallback is picked from the record id so a property keeps the same
//! photograph between visits instead of shuffling on every page load.

/// Hotel exteriors, pools and lobbies.
const HOTELS: &[&str] = &[
    "photo-1566073771259-6a8506099945",
    "photo-1571003123894-1f0594d2b5d9",
    "photo-1520250497591-112f2f40a3f4",
    "photo-1551882547-ff40c63fe5fa",
    "photo-1568084680786-a84f91d1153c",
    "photo-1517840901100-8179e982acb7",
    "photo-1613490493576-7fde63acd811",
    "photo-1621293954908-907159247fc8",
    "photo-1445019980597-93fa8acb246c",
    "photo-1582719508461-905c673771fd",
    "photo-1584132967334-10e028bd69f7",
];

/// Room interiors, used when a room has no photograph of its own.
const ROOMS: &[&str] = &[
    "photo-1505693416388-ac5ce068fe85",
    "photo-1512918728675-ed5a9ecdebfd",
    "photo-1560185893-a55cbc8c57e8",
    "photo-1590490360182-c33d57733427",
    "photo-1587985064135-0366536eab42",
    "photo-1611892440504-42a792e24d32",
    "photo-1618773928121-c32242e63f39",
    "photo-1631049307264-da0ec9d70304",
    "photo-1578683010236-d716f9a3f461",
    "photo-1596394516093-501ba68a0ba6",
];

/// Room photographs chosen to match a room type, so a "Family Room" does not
/// get a picture of a single bed. Matched on a lowercase substring of the
/// room's `room_type`, first hit wins.
const ROOM_BY_TYPE: &[(&str, &str)] = &[
    ("suite", "photo-1611048267451-e6ed903d4a38"),
    ("executive", "photo-1560448204-e02f11c3d0e2"),
    ("family", "photo-1554995207-c18c203602cb"),
    ("twin", "photo-1540518614846-7eded433c457"),
    ("single", "photo-1631049307264-da0ec9d70304"),
    ("deluxe", "photo-1590490360182-c33d57733427"),
    ("king", "photo-1505693416388-ac5ce068fe85"),
    ("double", "photo-1618773928121-c32242e63f39"),
    ("standard", "photo-1611892440504-42a792e24d32"),
    ("studio", "photo-1522708323590-d24dbb6b0267"),
];

/// Destination artwork. These are representative property photographs rather
/// than photographs of the city itself — coastal shots for the coastal
/// capitals, highland ones for the Ethiopian plateau — so nothing on the tile
/// contradicts where it points.
const CITIES: &[(&str, &str)] = &[
    ("addis ababa", "photo-1445019980597-93fa8acb246c"),
    ("dire dawa", "photo-1613490493576-7fde63acd811"),
    ("bahir dar", "photo-1571003123894-1f0594d2b5d9"),
    ("hawassa", "photo-1551882547-ff40c63fe5fa"),
    ("nairobi", "photo-1568084680786-a84f91d1153c"),
    ("mombasa", "photo-1584132967334-10e028bd69f7"),
    ("mogadishu", "photo-1582719508461-905c673771fd"),
    ("hargeisa", "photo-1517840901100-8179e982acb7"),
    ("djibouti", "photo-1520250497591-112f2f40a3f4"),
    ("dhaka", "photo-1621293954908-907159247fc8"),
];

/// Wraps a bare Unsplash photo id into a sized, cropped delivery URL.
fn sized(photo: &str, w: u32) -> String {
    format!("https://images.unsplash.com/{photo}?q=80&w={w}&auto=format&fit=crop")
}

/// Stable index into a pool. Ids are small and sequential, so a plain modulo
/// would hand consecutive hotels neighbouring pictures; the multiply spreads
/// them out.
fn pick(id: i64, len: usize) -> usize {
    ((id.unsigned_abs().wrapping_mul(2_654_435_761)) % len as u64) as usize
}

/// `true` when the API actually gave us a usable image URL.
fn real(url: Option<&str>) -> Option<String> {
    url.map(str::trim)
        .filter(|u| u.starts_with("http"))
        .map(str::to_string)
}

/// Cover photograph for a hotel: its own logo when it has one, else a stable
/// pick from the pool.
pub fn hotel_cover(id: i64, logo: Option<&str>, w: u32) -> String {
    real(logo).unwrap_or_else(|| sized(HOTELS[pick(id, HOTELS.len())], w))
}

/// Photograph for a room: its own primary image when set, else one matched to
/// the room type, else a stable pick from the pool.
pub fn room_image(id: i64, own: Option<&str>, room_type: Option<&str>, w: u32) -> String {
    if let Some(u) = real(own) {
        return u;
    }
    let t = room_type.unwrap_or_default().to_lowercase();
    let photo = ROOM_BY_TYPE
        .iter()
        .find(|(key, _)| t.contains(key))
        .map(|(_, p)| *p)
        .unwrap_or_else(|| ROOMS[pick(id, ROOMS.len())]);
    sized(photo, w)
}

/// Artwork for a destination tile.
pub fn city_image(city: &str, own: Option<&str>, w: u32) -> String {
    if let Some(u) = real(own) {
        return u;
    }
    let key = city.trim().to_lowercase();
    let photo = CITIES
        .iter()
        .find(|(name, _)| key.contains(name))
        .map(|(_, p)| *p)
        .unwrap_or_else(|| HOTELS[pick(key.len() as i64 + 3, HOTELS.len())]);
    sized(photo, w)
}

/// A gallery for a hotel that has uploaded none of its own, so the photo mosaic
/// and its lightbox still have something to show. Always five, always the same
/// five for a given hotel, starting from that hotel's cover.
pub fn hotel_gallery(id: i64, own: Vec<String>) -> Vec<String> {
    if !own.is_empty() {
        return own;
    }
    let start = pick(id, HOTELS.len());
    let mut out: Vec<String> = (0..3)
        .map(|i| sized(HOTELS[(start + i) % HOTELS.len()], 1400))
        .collect();
    let r = pick(id, ROOMS.len());
    out.extend((0..2).map(|i| sized(ROOMS[(r + i) % ROOMS.len()], 1400)));
    out
}
