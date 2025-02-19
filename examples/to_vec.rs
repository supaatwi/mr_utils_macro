use mr_utils_macro::ToVec;

// Custom formatting functions
fn add_status_prefix(value: &str) -> String {
    format!("Status: {}", value)
}

fn format_location(value: &str) -> String {
    format!("Location: {}", value)
}
fn format_lng(value: &str) -> String {
    format!("Lng: {}", value)
}

#[derive(ToVec)]
struct RecordDetail {
    #[to_vec(deserialize_with = "add_status_prefix")]
    status: String,
    #[to_vec(deserialize_with = "format_location")]
    geofence: String,
    latitude: f64,
    #[to_vec(format = "{:.2}", deserialize_with = "format_lng")]
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

    let record_vec = vec![ 
        RecordDetail {
            status: "Active".to_string(),
            geofence: "Zone A".to_string(),
            latitude: 37.7749,
            longitude: -122.4194,
            created_at: "2024-02-18T10:00:00Z".to_string(),
            desc: None
        },
        RecordDetail {
            status: "Active".to_string(),
            geofence: "Zone A".to_string(),
            latitude: 37.7749,
            longitude: -122.4194,
            created_at: "2024-02-18T10:00:00Z".to_string(),
            desc: None
        }
    ];
   
    #[cfg(feature = "waylar")]
    {
      
        println!("With number: {:?}", record_vec.to_with_number_list(Some(&["status", "geofence"])));
        println!("to_first_row_with : {:?}", record_vec.to_first_row_with(None, "Header"));
    }


    // Get all fields
    let all = record.to_vec(None);
    println!("All fields: {:?}", all);
    // Output: ["Status: Active", "Location: Zone A", "37.7749", "-122.42", "2024-02-18T10:00:00Z", "-"]

    // Get specific fields
    let location = record.to_vec(Some(&["status", "geofence"]));
    println!("Location info: {:?}", location);
    // Output: ["Status: Active", "Location: Zone A"]
}