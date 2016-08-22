
RUN

cargo run --release -- -w 100 -c 150 --user guest --password guest --url "http://host:port/" --from "-1h" --ignore-last-none 3 carbon.api.latency
