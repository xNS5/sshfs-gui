use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
struct Config {
    input_type: String,
    name: String,
}

struct InputConfig {
    font_size: String,
    width: String,
    height: String,
    placeholder_text: String
}

