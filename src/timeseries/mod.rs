extern crate json;

pub struct DataPoint {
    pub timestamp  :i64,
    pub value      :Option<f64>
}

impl ::std::fmt::Display for DataPoint {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let _ = write!(f, "<DataPoint:{}|", self.timestamp);
        write!(f, "{}>", match self.value {
            Some(i)  => i.to_string(),
            None     => "null".to_string(),
        })
    }
}

impl DataPoint {
    pub fn value_string(&self) -> String {
        match self.value {
            Some(i) => i.to_string(),
            None    => "null".to_string(),
        }
    }
}

pub struct MetricResult {
    pub metric     :String,
    pub datapoints :Vec<DataPoint>,
}

pub type MetricResultList = Vec<MetricResult>;

pub fn fetch_metric(s: &self::json::JsonValue) -> MetricResult {

    let mut mr :MetricResult = MetricResult{
        metric: "".to_string(),
        datapoints: vec!(),
    };

    mr.metric = s["target"].as_str().unwrap().to_string();

    for dp in s["datapoints"].members() {
        let data_point = DataPoint{
            timestamp: dp[1].as_i64().unwrap(),
            value: dp[0].as_f64(),
        };
        mr.datapoints.push(data_point);
    }

    mr
}

impl MetricResult {
    pub fn parse(body: String) -> MetricResultList {
        let mut result: MetricResultList = Vec::new();
        let parsed = json::parse(&body)
            .expect("The response is not a JSON string.");
        for s in parsed.members() {
            let mr = fetch_metric(&s);
            result.push(mr);
        }
        result
    }
}
