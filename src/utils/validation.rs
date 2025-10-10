pub fn validate_timezone(tz: &str) -> bool {
    // Simple timezone format validation
    if tz.len() < 3 || tz.len() > 6 {
        return false;
    }
    
    if !tz.starts_with('+') && !tz.starts_with('-') {
        return false;
    }
    
    let hours_part = &tz[1..3];
    if !hours_part.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }
    
    let hours: i32 = match hours_part.parse() {
        Ok(h) => h,
        Err(_) => return false,
    };
    
    if hours > 14 {
        return false;
    }
    
    if tz.len() > 3 {
        if !tz.chars().nth(3).map_or(false, |c| c == ':') {
            return false;
        }
        
        if tz.len() == 6 {
            let minutes_part = &tz[4..6];
            if !minutes_part.chars().all(|c| c.is_ascii_digit()) {
                return false;
            }
            
            let minutes: i32 = match minutes_part.parse() {
                Ok(m) => m,
                Err(_) => return false,
            };
            
            if minutes >= 60 {
                return false;
            }
        }
    }
    
    true
}