use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug, Clone, Default)]
#[command(name = "handy", about = "Handy - Speech to Text")]
pub struct CliArgs {
    /// Select the transcription language, or use `auto` for language detection.
    /// The selection is persisted and forwarded to an already-running instance.
    #[arg(long, value_name = "CODE", value_parser = parse_language_code)]
    pub language: Option<String>,

    /// Start with the main window hidden
    #[arg(long)]
    pub start_hidden: bool,

    /// Disable the system tray icon
    #[arg(long)]
    pub no_tray: bool,

    /// Toggle transcription on/off (sent to running instance)
    #[arg(long)]
    pub toggle_transcription: bool,

    /// Toggle transcription with post-processing on/off (sent to running instance)
    #[arg(long)]
    pub toggle_post_process: bool,

    /// Cancel the current operation (sent to running instance)
    #[arg(long)]
    pub cancel: bool,

    /// Enable debug mode with verbose logging
    #[arg(long)]
    pub debug: bool,

    /// Transcribe this WAV (16 kHz mono) headlessly and exit. Runs the same
    /// batch transcription path as the app — no mic, no silence removal, no
    /// download (the model must already be installed). Long audio is still
    /// chunked at pauses, as in the app.
    #[arg(short = 'f', long, value_name = "WAV")]
    pub transcribe_file: Option<PathBuf>,

    /// Model id to load for --transcribe-file (default: the selected model).
    #[arg(long)]
    pub model: Option<String>,

    /// Hard-select the compute device for --transcribe-file by its registry
    /// index (see --list-devices). Omit to use the persisted accelerator
    /// setting. transcribe-cpp (whisper-family) models only.
    #[arg(long, value_name = "N")]
    pub device_index: Option<usize>,

    /// List the transcribe-cpp compute devices (with indices) and exit.
    #[arg(long)]
    pub list_devices: bool,

    /// List the available models (with ids) and exit. Pass an id to --model.
    /// Honors --json for machine-readable output.
    #[arg(long)]
    pub list_models: bool,

    /// Repeat the transcription N times (best_ms reports the fastest run).
    #[arg(long, value_name = "N")]
    pub repeat: Option<usize>,

    /// Emit --transcribe-file results as JSON.
    #[arg(long)]
    pub json: bool,
}

pub fn parse_language_code(value: &str) -> Result<String, String> {
    if value.eq_ignore_ascii_case("auto") {
        return Ok("auto".to_string());
    }

    let mut subtags = value.split('-');
    let primary = subtags.next().unwrap_or_default();
    if !(2..=8).contains(&primary.len())
        || !primary.bytes().all(|b| b.is_ascii_alphabetic())
    {
        return Err("language must be `auto` or a BCP-47 language code".to_string());
    }
    if subtags.any(|subtag| {
        subtag.is_empty()
            || subtag.len() > 8
            || !subtag.bytes().all(|b| b.is_ascii_alphanumeric())
    }) {
        return Err("language must be `auto` or a BCP-47 language code".to_string());
    }

    Ok(value.to_string())
}

#[cfg(test)]
mod tests {
    use super::{parse_language_code, CliArgs};
    use clap::Parser;

    #[test]
    fn parses_language_argument() {
        let args = CliArgs::try_parse_from(["handy", "--language", "zh-Hant"])
            .expect("language argument should parse");

        assert_eq!(args.language.as_deref(), Some("zh-Hant"));
    }

    #[test]
    fn validates_language_codes() {
        assert_eq!(parse_language_code("AUTO").unwrap(), "auto");
        assert_eq!(parse_language_code("fr").unwrap(), "fr");
        assert_eq!(parse_language_code("zh-Hant").unwrap(), "zh-Hant");
        assert!(parse_language_code("bad_code").is_err());
        assert!(parse_language_code("-fr").is_err());
    }
}
