use std::fmt::{Debug, Formatter};

mod ffprobe;

pub struct ProbeBuilder {
    command: tokio::process::Command,
    path: String,
}

impl ProbeBuilder {
    pub fn new(path: &str) -> Self {
        let mut command = tokio::process::Command::new("ffprobe");

        command.arg("-v")
            .arg("quiet")
            .arg("-show_error")
            .arg("-print_format")
            .arg("xml")
            .arg("-noprivate");

        Self {
            command,
            path: path.to_string(),
        }
    }

    pub fn show_format(self) -> Self {
        let mut builder = self;
        builder.command.arg("-show_format");
        builder
    }
    pub fn show_frames(self) -> Self {
        let mut builder = self;
        builder.command.arg("-show_frames");
        builder
    }
    pub fn show_packets(self) -> Self {
        let mut builder = self;
        builder.command.arg("-show_packets");
        builder
    }
    pub fn show_programs(self) -> Self {
        let mut builder = self;
        builder.command.arg("-show_programs");
        builder
    }
    pub fn show_streams(self) -> Self {
        let mut builder = self;
        builder.command.arg("-show_streams");
        builder
    }
    pub fn show_chapters(self) -> Self {
        let mut builder = self;
        builder.command.arg("-show_chapters");
        builder
    }
    pub fn show_program_version(self) -> Self {
        let mut builder = self;
        builder.command.arg("-show_program_version");
        builder
    }
    pub fn show_library_versions(self) -> Self {
        let mut builder = self;
        builder.command.arg("-show_library_versions");
        builder
    }
    pub fn show_pixel_formats(self) -> Self {
        let mut builder = self;
        builder.command.arg("-show_pixel_formats");
        builder
    }

    pub async fn execute(self) -> Result<ffprobe::FFProbeRaw, Error> {
        let mut builder = self;
        builder.command.arg(&builder.path);

        let output = builder.command
            .output()
            .await
            .map_err(|err| {
                Error::CommandError(err)
            })?;

        let result = serde_xml_rs::from_reader::<_, ffprobe::FFProbeRaw>(output.stdout.as_slice())
            .map_err(|err| {
                Error::DeserializeError(err)
            });

        match result {
            Ok(res) => {
                if let Some(err) = res.error {
                    Err(Error::FFProbeError(err))
                } else {
                    Ok(res)
                }
            }
            Err(err) => { Err(err) }
        }
    }
}

#[derive(Debug)]
pub enum Error {
    CommandError(std::io::Error),
    FFProbeError(ffprobe::FFProbeError),
    DeserializeError(serde_xml_rs::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::CommandError(err) => {
                write!(f, "Could not execute ffprobe: {}", err)
            }
            Error::FFProbeError(err) => {
                write!(f, "{} ({})", err.string, err.code)
            }
            Error::DeserializeError(err) => {
                write!(f, "Could not deserialize ffprobe output: {}", err)
            }
        }
    }
}

impl std::error::Error for Error {}