use clap::{App, Arg};

fn main() {
    let matches = App::new("producer")
        .version("0.1")
        .author("gensmusic <gensmusic@163.com>")
        .about("kafka producer")
        .arg(
            Arg::new("brokers")
                .long("brokers")
                .value_name("brokers")
                .default_value("127.0.0.1:9092")
                .about("brokers of kafka")
                .takes_value(true),
        )
        .arg(
            Arg::new("topic")
                .value_name("topic")
                .about("topic you want to produce")
                .required(true)
                .index(1),
        )
        .get_matches();



    println!("{:?}", matches);
}
