use mr_utils_macro::ToVec;


// Struct for database record
#[derive(ToVec)]
struct RecordDetail {
    status: String,
    geofence: String,
    latitude: f64,
    #[to_vec(format = "{:.2}")]
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

    // // Get location data
    let location = record.to_vec(Some(&["geofence", "latitude", "longitude", "desc"]));
    println!("Location: {:?}", location);
    // // Output: ["37.7749", "-122.4194"]

}