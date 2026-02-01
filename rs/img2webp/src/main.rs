use anyhow::{Context, Result};
use std::env;
use webp_anim::{read_image, AnimationEncoder, EncoderOptions, FrameConfig};

fn main() -> Result<()> {
    env_logger::init();

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        Help();
        return Ok(());
    }

    // Early check for help/version
    for arg in &args[1..] {
        if arg == "-h" || arg == "-help" {
            Help();
            return Ok(());
        }
        if arg == "-version" {
            println!("WebP Encoder version: (linked via libwebp-sys)");
            println!("Rust img2webp 1.0.0");
            return Ok(());
        }
    }

    let mut options = EncoderOptions::default();
    let mut global_frame_config = FrameConfig::default();
    let mut output_path = String::new();
    let mut loop_count = 0;
    let mut reverse_frames = false;
    let mut ping_pong = false;

    // First pass: Global options
    let mut i = 1;
    let mut parsed_args = vec![false; args.len()];
    while i < args.len() {
        match args[i].as_str() {
            "-o" => {
                if i + 1 < args.len() {
                    output_path = args[i + 1].clone();
                    parsed_args[i] = true;
                    parsed_args[i + 1] = true;
                    i += 1;
                }
            }
            "-kmin" => {
                if i + 1 < args.len() {
                    options.kmin = args[i + 1].parse().unwrap_or(0);
                    parsed_args[i] = true;
                    parsed_args[i + 1] = true;
                    i += 1;
                }
            }
            "-kmax" => {
                if i + 1 < args.len() {
                    options.kmax = args[i + 1].parse().unwrap_or(0);
                    parsed_args[i] = true;
                    parsed_args[i + 1] = true;
                    i += 1;
                }
            }
            "-loop" => {
                if i + 1 < args.len() {
                    loop_count = args[i + 1].parse().unwrap_or(0);
                    parsed_args[i] = true;
                    parsed_args[i + 1] = true;
                    i += 1;
                }
            }
            "-near_lossless" => {
                if i + 1 < args.len() {
                    global_frame_config.near_lossless = args[i + 1].parse().unwrap_or(100);
                    parsed_args[i] = true;
                    parsed_args[i + 1] = true;
                    i += 1;
                }
            }
            "-min_size" => {
                options.minimize_size = true;
                parsed_args[i] = true;
            }
            "-mixed" => {
                options.allow_mixed = true;
                global_frame_config.lossy = true;
                parsed_args[i] = true;
            }
            "-sharp_yuv" => {
                global_frame_config.use_sharp_yuv = true;
                parsed_args[i] = true;
            }
            "-reverse" => {
                reverse_frames = true;
                parsed_args[i] = true;
            }
            "-pingpong" => {
                ping_pong = true;
                parsed_args[i] = true;
            }
            "-v" => {
                options.verbose = true;
                parsed_args[i] = true;
            }
            "-alpha_q" => {
                if i + 1 < args.len() {
                    global_frame_config.alpha_quality = args[i + 1].parse().unwrap_or(100);
                    parsed_args[i] = true;
                    parsed_args[i + 1] = true;
                    i += 1;
                }
            }
            "-alpha_method" => {
                if i + 1 < args.len() {
                    global_frame_config.alpha_compression = args[i + 1].parse().unwrap_or(1);
                    parsed_args[i] = true;
                    parsed_args[i + 1] = true;
                    i += 1;
                }
            }
            "-alpha_filter" => {
                if i + 1 < args.len() {
                    global_frame_config.alpha_filtering = args[i + 1].parse().unwrap_or(1);
                    parsed_args[i] = true;
                    parsed_args[i + 1] = true;
                    i += 1;
                }
            }
            _ => {} // This is a comment, not code that needs escaping
        }
        i += 1;
    }

    // Second pass: Collect files and frame configs
    #[derive(Clone)]
    struct FrameJob {
        path: String,
        config: FrameConfig,
    }
    let mut jobs = Vec::new();
    let mut current_config = global_frame_config;

    i = 1;
    while i < args.len() {
        if parsed_args[i] {
            i += 1;
            continue;
        }

        let arg = &args[i];
        if arg.starts_with('-') {
            match arg.as_str() {
                "-lossy" => current_config.lossy = true,
                "-lossless" => current_config.lossy = false,
                "-q" => {
                    if i + 1 < args.len() {
                        current_config.quality = args[i + 1].parse().unwrap_or(75.0);
                        i += 1;
                    }
                }
                "-m" => {
                    if i + 1 < args.len() {
                        current_config.method = args[i + 1].parse().unwrap_or(4);
                        i += 1;
                    }
                }
                "-d" => {
                    if i + 1 < args.len() {
                        current_config.duration = args[i + 1].parse().unwrap_or(100);
                        i += 1;
                    }
                }
                "-exact" => current_config.exact = true,
                "-noexact" => current_config.exact = false,
                _ => {
                    eprintln!("Unknown option: {}", arg);
                }
            }
        } else {
            jobs.push(FrameJob {
                path: arg.clone(),
                config: current_config.clone(),
            });
        }
        i += 1;
    }

    if jobs.is_empty() {
        eprintln!("No input file(s) for generating animation!");
        Help();
        return Ok(());
    }

    // SEQUENCE MANIPULATION
    if reverse_frames {
        jobs.reverse();
    } else if ping_pong {
        let mut backward = jobs.clone();
        backward.reverse();
        if backward.len() > 2 {
            backward.remove(0);
            backward.pop();
        }
        jobs.extend(backward);
    }

    let mut encoder: Option<AnimationEncoder> = None;
    let mut pic_num = 0;
    let mut timestamp_ms = 0;

    for job in jobs {
        let img = read_image(&job.path).context(format!("Failed to read image: {}", job.path))?;

        if encoder.is_none() {
            let enc = AnimationEncoder::new(img.width() as i32, img.height() as i32, &options)?;
            encoder = Some(enc);
        }

        if let Some(enc) = &mut encoder {
            if options.verbose {
                eprintln!(
                    "Added frame #{} at time {} (file: {})",
                    pic_num, timestamp_ms, job.path
                );
            }
            enc.add_frame(&img, &job.config)?;
            timestamp_ms += job.config.duration;
            pic_num += 1;
        }
    }

    if let Some(mut enc) = encoder {
        let data = enc.assemble(loop_count)?;
        if !output_path.is_empty() {
            std::fs::write(&output_path, &data)?;
            if options.verbose {
                eprintln!("Output file: {} ", output_path);
            }
        } else {
            eprintln!("[no output file specified]");
        }
        eprintln!("[{} frames, {} bytes].", pic_num, data.len());
    }

    Ok(())
}

#[allow(non_snake_case)]
fn Help() {
    println!("Usage:");
    println!("  img2webp [file_options] [[frame_options] frame_file]... [-o webp_file]\n");
    println!("File-level options:");
    println!(" -min_size ............ minimize size");
    println!(" -mixed ............... use mixed lossy/lossless automatic mode");
    println!(" -loop <int> .......... loop count (default: 0, = infinite loop)");
    println!(" -near_lossless <int> . use near-lossless image preprocessing (0..100)");
    println!(" -alpha_q <int> ....... alpha quality (0..100), default 100");
    println!(" -alpha_method <int> .. alpha compression method (0..1), default 1");
    println!(" -alpha_filter <int> .. alpha filtering method (0..2), default 1");
    println!(" -reverse ............. reverse the order of input frames");
    println!(" -pingpong ............ forward then backward sequence for smooth looping");
    println!(" -swing ............... pendulum loop: 0 -> +25% -> 0 -> -25% -> 0");
    println!(" -v ................... verbose mode");
    println!(" -h ................... this help\n");
    println!("Per-frame options:");
    println!(" -d <int> ............. frame duration in ms (default: 100)");
    println!(" -lossless ............ use lossless mode (default)");
    println!(" -lossy ............... use lossy mode");
    println!(" -q <float> ........... quality");
    println!(" -m <int> ............. compression method (0=fast, 6=slowest)");
    println!(" -exact, -noexact ..... preserve or alter RGB values in transparent area");
}
