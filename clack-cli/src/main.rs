use clap::Parser;
use clack_core::{ClackConfig, ClackEngine};
use std::process;

#[derive(Parser, Debug)]
#[command(name = "htype")]
#[command(version = "1.0.0")]
#[command(about = "CLI driver for the Clack human typing simulator", long_about = None)]
struct Cli {
    #[arg(long, default_value_t = 60.0)]
    wpm: f64,

    #[arg(long, default_value_t = 0.15)]
    jitter: f64,

    #[arg(long, default_value_t = 0.04)]
    error_rate: f64,

    #[arg(long, default_value_t = 0.85)]
    correction_rate: f64,

    #[arg(long)]
    no_errors: bool,

    #[arg(long)]
    seed: Option<u64>,

    #[arg(long, default_value_t = 500)]
    session_length: usize,

    #[arg(long)]
    no_fatigue: bool,

    #[arg(long, default_value_t = 5000)]
    max_pause: u64,

    #[arg(long, default_value_t = 0.015)]
    thinking_pause_prob: f64,

    #[arg(long)]
    state_output: bool,

    #[arg(long, hide = true)]
    generate_man: bool,
}

fn main() {
    let cli = Cli::parse();

    #[cfg(feature = "mangen")]
    {
        if cli.generate_man {
            use clap::CommandFactory;
            let out_dir = std::path::PathBuf::from("target/man");
            std::fs::create_dir_all(&out_dir).unwrap();
            let mut file = std::fs::File::create(out_dir.join("clack.1")).unwrap();
            let cmd = Cli::command();
            clap_mangen::Man::new(cmd).render(&mut file).unwrap();
            println!("Man page generated at target/man/clack.1");
            return;
        }
    }

    let mut config = ClackConfig::default();
    config.wpm = cli.wpm;
    config.jitter = cli.jitter;
    config.error_rate = cli.error_rate;
    config.correction_rate = cli.correction_rate;
    config.no_errors = cli.no_errors;
    config.seed = cli.seed;
    config.session_length = cli.session_length;
    config.no_fatigue = cli.no_fatigue;
    config.max_pause_ms = cli.max_pause;
    config.thinking_pause_prob = cli.thinking_pause_prob;
    config.state_output = cli.state_output;

    let mut engine = match ClackEngine::new(config) {
        Ok(e) => e,
        Err(_) => {
            eprintln!("Error initializing engine (invalid option).");
            process::exit(1);
        }
    };

    use std::io::{self, Read, Write};
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Arc;
    use std::time::Duration;

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        // Immediate halt if Ctrl+C pressed twice
        if !r.load(Ordering::SeqCst) {
            process::exit(0);
        }
        r.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");

    let mut stdin = io::stdin().lock();
    let mut stdout = io::stdout();
    let mut buf = [0u8; 1024];

    while running.load(Ordering::SeqCst) {
        match stdin.read(&mut buf) {
            Ok(0) => break, // EOF
            Ok(n) => {
                engine.feed(&buf[..n]);
                while let Some(event) = engine.next_event() {
                    if !running.load(Ordering::SeqCst) {
                        break;
                    }
                    if event.delay_ms > 0 {
                        std::thread::sleep(Duration::from_millis(event.delay_ms));
                    }
                    if cli.state_output {
                        if let Some(st) = event.state_transition {
                            eprintln!("STATE:{} PREV:{} WORD:{}", st.new_state, st.prev_state, st.word_count);
                        }
                    }
                    stdout.write_all(&event.bytes).ok();
                    stdout.flush().ok();
                }
            }
            Err(_) => {
                process::exit(2);
            }
        }
    }

    engine.finish();
    while let Some(event) = engine.next_event() {
        if event.delay_ms > 0 {
            std::thread::sleep(Duration::from_millis(event.delay_ms));
        }
        if cli.state_output {
            if let Some(st) = event.state_transition {
                eprintln!("STATE:{} PREV:{} WORD:{}", st.new_state, st.prev_state, st.word_count);
            }
        }
        stdout.write_all(&event.bytes).ok();
        stdout.flush().ok();
    }
}
