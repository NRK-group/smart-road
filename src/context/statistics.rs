pub struct Statistics {
    pub max_number: i32,
    pub max_velocity: i32,
    pub min_velocity: i32,
    pub longest_time: f32,
    pub shortest_time: f32,
    pub close_calls: i32,
}

impl Statistics {
    pub fn new() -> Self {
        Self {
            max_number: 0,
            max_velocity: 0,
            min_velocity: 0,
            longest_time: 0.0,
            shortest_time: f32::MAX,
            close_calls: 0,
        }
    }

    pub fn format_stats(self) -> String {
        format!("Number of cars: {}\nMax velocity: {}\nMin Velocity: {}\nLongest Car: {}\nSlowest Time: {}\n Close calls: {}", self.max_number, self.max_velocity, self.min_velocity, if self.shortest_time != f32::MAX { self.shortest_time.to_string()} else {"N/A".to_string()}, if self.longest_time != 0.0 { self.longest_time.to_string()} else {"N/A".to_string()}, self.close_calls)
    }
}

pub fn stat_times(v: Vec<(f32, f32, f32)>) -> (Option<f32>, Option<f32>) {
    let mut h_result = -1.0;
    let mut l_result = f32::MAX;
    for t in v {
        if t.0 > h_result {
            h_result = t.0
        }
        if t.0 < l_result && t.0 > 0.0 {
            l_result = t.0
        }

        if t.1 > h_result {
            h_result = t.1
        }
        if t.1 < l_result && t.1 > 0.0 {
            l_result = t.1
        }

        if t.2 > h_result {
            h_result = t.2
        }
        if t.2 < l_result && t.2 > 0.0 {
            l_result = t.2
        }
    }

    let mut result = (None, None);
    if h_result != -1.0 {
        result.0 = Some(h_result)
    }

    if l_result != f32::MAX && l_result > 0.0 {
        result.1 = Some(l_result)
    }
    result
}
