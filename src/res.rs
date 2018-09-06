
use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
    static ref FILES: Mutex<HashMap<String, &'static [u8]>> = {
        let mut m = HashMap::new();
        Mutex::new(m)
    };    
}
