#[macro_use]
extern crate lazy_static;
extern crate rustc_serialize;

mod config;
mod timeseries;
mod graphiteapi;
mod nagios;

fn main() {
    let body = graphiteapi::http_fetch();
    let mrl = timeseries::MetricResult::parse(body);

    if mrl.len() == 0 {
        println!("no such metric.");
        nagios::exit(nagios::EXIT_UNKNOWN);
    }

    for mr in &mrl {
        check_metric_result(&mr);
    }
}

fn check_metric_result(mr: &timeseries::MetricResult) {
    let flag_ignore_last_none = config::ARGS.flag_ignore_last_none;
    let start: usize = 0;
    let mut end: usize = mr.datapoints.len() - 1;
    // Calc end position
    {
        let stop_pos = end - flag_ignore_last_none as usize;
        let mut pos = end;
        while pos > stop_pos {
            if mr.datapoints[pos].value != None {
                break
            }
            pos = pos - 1;
        }
        end = pos;
    }
    // Check
    let mut is_warning: bool = true;
    let mut is_critical: bool = true;
    for i in start..end {
        let dp = &mr.datapoints[i];
        match dp.value {
            None => {
                if ! config::ARGS.flag_ignore_none {
                    println!("the data point {} is null.", dp.timestamp);
                    nagios::exit(nagios::EXIT_CRITICAL)
                }
            },
            Some(i) => {
                if i < config::ARGS.flag_warning {
                    is_warning = false;
                }
                if i < config::ARGS.flag_critical {
                    is_critical = false;
                }
            },
        }
    }
    // Result
    if is_critical {
        println!("The last value is {}.",
                 &mr.datapoints[end - 1].value_string());
        nagios::exit(nagios::EXIT_CRITICAL);
    } else if is_warning {
        println!("The last value is {}.",
                 &mr.datapoints[end - 1].value_string());
        nagios::exit(nagios::EXIT_WARNING);
    } else {
        println!("OK");
        nagios::exit(nagios::EXIT_OK);
    }
}
