use mr_utils_macro::ToVec;

// Custom formatting functions
fn add_status_prefix(value: &str) -> String {
    format!("Status: {}", value)
}

fn format_location(value: &str) -> String {
    format!("Location: {}", value)
}

#[derive(ToVec)]
struct RecordDetail {
    #[to_vec(deserialize_with = "add_status_prefix")]
    status: String,
    #[to_vec(deserialize_with = "format_location")]
    geofence: String,
    latitude: f64,
    #[to_vec(format = "{:.2}", )]
    longitude: f64,
    created_at: String,
    #[to_vec(default = "-")]
    desc: Option<String>
}

fn main() {
    let record = RecordDetail {
        status: "Active".to_string(),
        geofence: "Zone A".to_string(),
        latitude: 37.7749,
        longitude: -122.4194,
        created_at: "2024-02-18T10:00:00Z".to_string(),
        desc: None
    };

    // Get all fields
    let all = record.to_vec(None);
    println!("All fields: {:?}", all);
    // Output: ["Status: Active", "Location: Zone A", "37.7749", "-122.42", "2024-02-18T10:00:00Z", "-"]

    // Get specific fields
    let location = record.to_vec(Some(&["status", "geofence"]));
    println!("Location info: {:?}", location);
    // Output: ["Status: Active", "Location: Zone A"]
}