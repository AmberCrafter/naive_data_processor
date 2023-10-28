use chrono::NaiveDateTime;

#[derive(Debug, Clone)]
pub enum DataType {
    Datetime(NaiveDateTime),
    Integer(i64),
    Float(f64),
    String(String),
    NULL,
}


pub fn data_parser_format(data_format: &Vec<String>, s: &str) -> Vec<(String, DataType)> {
    let mut res = Vec::new();
    let ele = s.split(',').collect::<Vec<_>>();

    for (key, val) in data_format.iter().zip(ele) {
        match key.to_lowercase().as_str() {
            "datetime" | "datetimelst" | "datetimeutc" => {
                res.push((
                    key.to_string(), 
                    DataType::Datetime(
                        NaiveDateTime::parse_from_str(val, "%Y-%m-%dT%H:%M:%S").expect(&format!("Unspport datetime format: {}", val))
                    )
                ));
            },
            _ => {
                res.push((
                    key.to_string(), 
                    DataType::Float(
                        val.parse::<f64>().expect(&format!("Unspport data format: {}", val))
                    )
                ));
            }
        }
    }
    res
}

pub fn data_parser_key_value(s: &str) -> Vec<(String, DataType)> {
    let mut res = Vec::new();
    let tmp = s.split(',').collect::<Vec<_>>();

    for ele in tmp {
        let (key, val) = ele.split_at(ele.find('=').expect(&format!("Invalid data: {}", ele)));
        let v = val[1..].parse::<f64>().expect(&format!("Unspport data format: {}", ele));
        res.push((
            key.to_string(), 
            DataType::Float(v)
        ));
    }
    res
}
