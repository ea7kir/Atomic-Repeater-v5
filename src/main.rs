// Atomic-Repeater-v5 - rust version

use glob;
use serde::{Deserialize, Serialize};
use toml;

fn main() {
    #[derive(Serialize, Deserialize)]
    struct Config {
        repeater: Repeater,
        carousel: Carousel,
        batc: Batc,
        rtmp: Rtmp,
        srt: Srt,
        receiver: Receiver,
        transmitter: Transmitter,
    }

    #[derive(Serialize, Deserialize)]
    struct Repeater {
        title: String,
        sub_title: String,
        batc_text: String,
        batc_link: String,
        rtmp_text: String,
        rtmp_link: String,
        srt_text: String,
        srt_link: String,
        freq_text: String,
        freq_in_out: String,
        access_info: String,
    }

    #[derive(Serialize, Deserialize)]
    struct Carousel {
        enabled: bool,
        media_path: String,
        slides_per_ident: u16,
        seconds_per_slide: u16,
    }

    #[derive(Serialize, Deserialize)]
    struct Batc {
        enabled: bool,
        dummy: String,
    }

    #[derive(Serialize, Deserialize)]
    struct Rtmp {
        enabled: bool,
        dummy: String,
    }

    #[derive(Serialize, Deserialize)]
    struct Srt {
        enabled: bool,
        dummy: String,
    }

    #[derive(Serialize, Deserialize)]
    struct Receiver {
        enabled: bool,
        dummy: String,
    }

    #[derive(Serialize, Deserialize)]
    struct Transmitter {
        enabled: bool,
        dummy: String,
    }

    let config = Config {
        repeater: Repeater {
            title: "Málaga ED7TMP Repeater".to_string(),
            sub_title: "Experimental v4".to_string(),
            batc_text: "Watch on the BATC Streamer".to_string(),
            batc_link: "http://batc.org.uk/live/ed7tmp".to_string(),
            rtmp_text: "Watch with VLC or ffplay".to_string(),
            rtmp_link: "rtmp://rtmp.batc.org.uk/live/ed7tmp".to_string(),
            srt_text: "Watch the SRT feed".to_string(),
            srt_link: "srt:/t.b.a.".to_string(),
            freq_text: "Input/Output Frequencies".to_string(),
            freq_in_out: "input: ####.### MHz output: ####.### MHz".to_string(),
            access_info: "access keys will be available from ea7kir@icloud.com".to_string(),
        },
        carousel: Carousel {
            enabled: true,
            media_path: "../media".to_string(),
            slides_per_ident: 5,
            seconds_per_slide: 10,
        },
        batc: Batc {
            enabled: false,
            dummy: "dummy".to_string(),
        },
        rtmp: Rtmp {
            enabled: false,
            dummy: "dummy".to_string(),
        },
        srt: Srt {
            enabled: false,
            dummy: "dummy".to_string(),
        },
        receiver: Receiver {
            enabled: false,
            dummy: "dummy".to_string(),
        },
        transmitter: Transmitter {
            enabled: false,
            dummy: "dummy".to_string(),
        },
    };

    let toml_str = toml::to_string(&config).unwrap();
    println!("{}", toml_str);

    // TODO: implement config read & write in a module

    // TODO: implement carousel algorithm
    // read list of slide
    // config::read();
    for entry in glob("../media/slides/*.png")? {
        println!("{}", entry?.display());
    }
}
