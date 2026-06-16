use rsubs_lib::VTT;
use std::fs;
use std::io::Seek;
use std::io::SeekFrom;
/**
 * This module provides functionality for downloading videos from a given URL,
 * extracting subtitles in specified languages, and serving the video and subtitle files through HTTP endpoints.
 */
// Standard library imports for file handling, process execution, and path manipulation.
use std::path::PathBuf;
use std::process::Command;

// Actix-web imports for building the REST API endpoints.
use actix_web::{HttpRequest, HttpResponse, Responder, http::header};

// Local module imports for handling video processing and errors.
use tokio::io::AsyncReadExt;

use crate::common::database;
// Import the custom error type for handling errors in video processing.
use crate::common::{
    context::Context,
    errors::{Error, IOInfo},
};

use crate::database::videos::{Video, VideoDatabase};

/**
 * Downloads a video from the given URL and extracts subtitles in the specified languages.
 * It uses the yt-dlp command-line tool to perform the download and subtitle extraction.
 * The function generates a unique identifier for each download to avoid conflicts and organizes the downloaded files in a structured directory.
 * The resulting Video struct contains the URL, paths to the downloaded video and subtitles, and the list of languages for which subtitles were extracted.
 * # Arguments
 * * `url` - A string slice that holds the URL of the video to be downloaded.
 * * `languages` - A slice of string slices that holds the languages for which subtitles should be extracted.
 * # Returns
 * * `Result<Video, LlaasError>` - A result containing the Video struct if the download and subtitle extraction are successful,
 * * or a LlaasError if any step of the process fails.
 */
pub async fn download(_context: &Context, url: &str, languages: &[&str]) -> Result<Video, Error> {
    // Get the video database from the context.
    let database: &dyn VideoDatabase = _context;

    // Get the uid based on a hash of the URL.
    let uid = &uuid::Uuid::new_v4().to_string();

    // Create the output directory for the downloaded video and subtitles.
    let output = directory(uid);

    println!(
        "Downloading video from URL '{}' to '{}'",
        url,
        output.to_str().unwrap()
    );

    // Execute the yt-dlp command to download the video and extract subtitles in the specified languages.
    // https://www.ditig.com/yt-dlp-cheat-sheet#embed-metadata-and-thumbnail
    // yt-dlp -f "mp4" --no-playlist --embed-metadata --embed-thumbnail --movflags +faststart --write-subs --sub-langs "es" "{utl}" -o "output.%(ext)s"
    // We show the progress of the download and subtitle extraction process by printing messages to the console.
    Command::new("yt-dlp")
        .args(&[
            "-f",
            "mp4",
            "--no-playlist",
            "--embed-metadata",
            "--embed-thumbnail",
            //"--movflags", "+faststart",
            "--write-subs",
            "--sub-langs",
            &languages.join(","),
            url,
            "-o",
            output.join("output.%(ext)s").to_str().unwrap(),
        ])
        .stdout(std::process::Stdio::inherit())
        .output()?;

    // Create a Video struct with the URL, paths to the downloaded video and subtitles,
    // and the list of languages for which subtitles were extracted.
    let video = read(output, uid, url, languages)?;

    // Upsert the video record in the database.
    database.upsert(&video).await?;

    // The Video struct is returned with the URL,
    // paths to the downloaded video and subtitles,
    // and the list of languages for which subtitles were extracted.
    Ok(video)
}

/**
 * Reads the downloaded video and subtitle files from the specified output directory and constructs a Video struct.
 * The function scans the output directory for files that match the expected naming pattern for the video and subtitle files,
 * extracts the relevant information such as the video file name, subtitle file names, and languages,
 * and returns a Video struct containing the URL, paths to the downloaded video and subtitles, and the list of languages for which subtitles were extracted.
 * # Arguments
 * * `output` - A PathBuf representing the directory where the downloaded video and subtitles are
 * stored, which is generated based on a unique identifier (UUID) for the download.
 * * `uid` - A UUID that serves as a unique identifier for the video resource,
 * * `url` - A string slice that holds the URL of the video that was downloaded,
 * * `languages` - A slice of string slices that holds the languages for which subtitles were extracted.
 * # Returns
 * * `Result<Video, LlaasError>` - A result containing the Video struct if the video and subtitle files are successfully read and processed,
 * * or a LlaasError if there are any issues in reading the files, extracting the information, or constructing the Video struct.
 */
fn read(output: PathBuf, uid: &str, url: &str, languages: &[&str]) -> Result<Video, Error> {
    let files: Vec<(String, bool)> = fs::read_dir(output)?
        .filter_map(|e| {
            let entry = e.ok()?;
            let path = entry.path();
            let file_name = path.file_name()?.to_str()?;

            // If file starts with output and ends with .mp4 or any like srt, vtt subtitles extension.
            if file_name.starts_with("output")
                && (file_name.ends_with(".mp4")
                    || file_name.ends_with(".vtt")
                    || file_name.ends_with(".srt"))
            {
                Some((file_name.to_string(), file_name.ends_with(".mp4")))
            } else {
                None
            }
        })
        .collect();

    // Get the video file name or return an error if the video file is not found in the output directory.
    let video = match files
        .iter()
        .find(|(name, is_video)| *is_video)
        .map(|(name, _)| name.clone())
    {
        Some(name) => name,
        None => {
            return Err(Error::IOError(IOInfo(
                format!(
                    "Video file not found in output directory for video with ID {}.",
                    uid
                ),
                None,
            )));
        }
    };

    // Get all the subtitle files.
    let subtitles: Vec<String> = files
        .iter()
        .filter_map(|(name, is_video)| if !*is_video { Some(name) } else { None })
        .map(|name| name.into())
        .collect();

    // Extract the languages for which subtitles were extracted based on the subtitle file names.
    let langauges = subtitles
        .iter()
        .filter_map(|name| {
            // Extract the language code from the subtitle file name.
            let parts: Vec<&str> = name.split('.').collect();
            if parts.len() >= 3 {
                Some(parts[1].to_string())
            } else {
                None
            }
        })
        .collect();

    // Create a Video struct.
    Ok(Video {
        id: Some(database::record("video", uid)),
        url: url.to_string(),
        video: video.into(),
        subtitles: subtitles,
        languages: langauges,
    })
}

/**
 * Retrieves the path to the downloaded video file for a given unique identifier (UUID).
 * The function constructs the path to the video file based on the directory structure defined in the `directory` function and checks if the file exists.
 * It returns a tuple containing the path to the video file as a string and a boolean indicating whether the file exists and is valid.
 *
 * Arguments
 * * `uid` - A UUID that serves as a unique identifier for the video resource, used to locate the corresponding video file on the server.
 * Returns
 * * `Option<String>` - An option containing the path to the video file as a string
 * * boolean indicating whether the file exists and is valid. If the file does not exist or is not valid, it returns None.
 */
pub fn subtitles(context: &Context, uid: &str, lang: &str) -> Result<String, Error> {
    match subtitle(context, uid, lang) {
        Ok(path) => Ok(std::fs::read_to_string(path)?),
        Err(e) => Err(e),
    }
}

/**
 * Generates an HTML view for a video with synchronized subtitle playback.
 * The function retrieves the subtitle file for the specified video and language,
 * parses it using rsubs_lib's WebVTT parser, and generates an HTML page that includes a video player with controls and a track for subtitles.
 * The generated HTML also includes a timeline of buttons for each subtitle cue, allowing users to jump to specific timestamps in the video.
 * When a button is clicked, it triggers a JavaScript function that jumps to the corresponding time in the video, enabling instant subtitle playback synchronization.
 *
 * Arguments
 * * `uid` - A UUID that serves as a unique identifier for the video resource,
 * * `lang` - A string slice that holds the language code for the subtitles to be displayed in the view.
 * Returns
 * * `Result<String, LlaasError>` - A result containing the generated HTML view as a string if successful,
 * * or a LlaasError if there is an error in retrieving the subtitle file, parsing
 * * the VTT content, or any other issues that may arise during the view generation process.
 */
pub fn view(context: &Context, uid: &str, lang: &str) -> Result<String, Error> {
    let file = subtitle(context, uid, lang)?;
    let content = std::fs::read_to_string(file)?;
    let vtt = VTT::parse(&content).map_err(|e| {
        Error::IOError(IOInfo(
            format!(
                "Failed to parse subtitle file for video {} in language {}: {}",
                uid, lang, e
            ),
            None,
        ))
    })?;

    // Generate HTML buttons for each subtitle cue, allowing users to jump to specific timestamps in the video.
    // Each button displays the timestamp and the subtitle text, and when clicked,
    // it triggers a JavaScript function to jump to the corresponding time in the video.
    let mut buttons = String::new();
    vtt.lines.iter().for_each(|cue| {
        let start_seconds = cue.start.hour() as f64 * 3600.0
            + cue.start.minute() as f64 * 60.0
            + cue.start.second() as f64
            + cue.start.nanosecond() as f64 / 1_000_000_000.0;
        let label_minutes = (start_seconds / 60.0).floor() as u64;
        let label_seconds = (start_seconds % 60.0).floor() as u64;
        let time_label = format!("{:02}:{:02}", label_minutes, label_seconds);
        let cue_text = cue.text.split_whitespace().collect::<Vec<_>>().join(" ");
        buttons.push_str(&format!(
            r#"<button onclick="jump_to({})">[{}] "{}"</button>"#,
            start_seconds, time_label, cue_text
        ));
    });

    // The generated HTML includes a video player with controls and a track for subtitles,
    // as well as a timeline of buttons for each subtitle cue.
    Ok(format!(
        r#"<!DOCTYPE html>
        <html>
        <head>
            <meta charset="utf-8">
            <title>Dynamic Video Streaming Engine</title>
            <style>
                body {{ font-family: sans-serif; margin: 40px; background: #121212; color: #fff; }}
                #subtitle-timeline {{ margin-top: 20px; max-height: 300px; overflow-y: auto; padding: 10px; background: #1e1e1e; border-radius: 6px; }}
                button {{ margin: 6px 0; padding: 10px; cursor: pointer; display: block; width: 100%; text-align: left; background: #2a2a2a; color: #fff; border: 1px solid #3a3a3a; border-radius: 4px; }}
                button:hover {{ background: #3a3a3a; }}
                video {{ border-radius: 6px; background: #000; }}
            </style>
        </head>
        <body>
            <h3>Instant Subtitle Playback Sync</h3>
            <video id="myVideo" controls width="640">
                <source src="/videos/{uid}.mp4" type="video/mp4">
                <track id="subTrack" src="/videos/{uid}/{lang}/subtitles.vtt" kind="subtitles" srclang="{lang}" label="{lang}" default>
            </video>

            <div id="subtitle-timeline">
                {}
            </div>

            <script>
                function jump_to(seconds) {{
                    const video = document.getElementById("myVideo");
                    video.currentTime = seconds; // Instantly triggers partial chunk range requests
                    video.play();
                }}
            </script>
        </body>
        </html>"#,
        buttons
    ))
}

/**
 * Streams the video file for a given unique identifier (UUID) in response to an HTTP request.
 * The function checks if the video file exists and is valid, and if so, it opens
 * the file and parses the HTTP Range header to determine the byte range requested by the client.
 * It then seeks the file pointer to the specified byte offset and creates a non-blocking streaming
 * payload to stream the requested portion of the video file back to the client with appropriate HTTP headers for content type, content range, and content length.
 * If the video file does not exist or is not valid, it returns a 404 Not Found response. If there are any errors in opening the file, reading metadata, or seeking the file pointer, it returns a 500 Internal Server Error response with an appropriate error message.
 * # Arguments
 * * `req` - The HttpRequest object representing the incoming HTTP request from the client,
 * * `uid` - A UUID that serves as a unique identifier for the video resource, used to locate the corresponding video file on the server.
 * # Returns
 * * `impl Responder` - An HTTP response that either streams the requested portion of the video file back to the client with appropriate headers if the file exists and is valid, or returns an
 * * error response if the file does not exist, is not valid, or if there are any issues in processing the request.
 */
pub fn stream(_context: &Context, req: HttpRequest, uid: String) -> impl Responder {
    let file_info = match video(&uid) {
        Ok(info) => info,
        Err(_) => {
            return HttpResponse::NotFound()
                .body(format!("Video with ID {} is not available.", uid));
        }
    };

    // 1. Open the file synchronously as initiated in your snippet
    let mut file = match std::fs::File::open(&file_info) {
        Ok(f) => f,
        Err(_) => {
            return HttpResponse::InternalServerError().body(format!(
                "Failed to open video file for video with ID {}!!",
                uid
            ));
        }
    };

    // 2. Get the full size of the file
    let metadata = match file.metadata() {
        Ok(m) => m,
        Err(_) => {
            return HttpResponse::InternalServerError().body("Failed to read video metadata.");
        }
    };
    let file_size = metadata.len();

    // 3. Parse the standard browser HTTP Range header
    let range_header = req
        .headers()
        .get(header::RANGE)
        .and_then(|v| v.to_str().ok());

    if let Some(range_str) = range_header {
        if let Some(range) = range_str.strip_prefix("bytes=") {
            let parts: Vec<&str> = range.split('-').collect();
            let start = parts
                .get(0)
                .and_then(|&s| s.parse::<u64>().ok())
                .unwrap_or(0);
            let end = parts
                .get(1)
                .and_then(|&s| s.parse::<u64>().ok())
                .unwrap_or(file_size - 1);

            // Guard against invalid byte boundaries
            if start >= file_size || end >= file_size || start > end {
                return HttpResponse::RangeNotSatisfiable()
                    .insert_header((header::CONTENT_RANGE, format!("bytes */{}", file_size)))
                    .finish();
            }

            let chunk_size = end - start + 1;

            // Seek the file pointer to the specific timestamp offset slice
            if file.seek(SeekFrom::Start(start)).is_err() {
                return HttpResponse::InternalServerError()
                    .body("Failed to seek file payload offset.");
            }

            // Convert standard file to a non-blocking streaming chunk payload
            let tokio_file = tokio::fs::File::from_std(file);
            let stream = tokio_util::io::ReaderStream::new(tokio_file.take(chunk_size));

            return HttpResponse::PartialContent()
                .insert_header((header::CONTENT_TYPE, "video/mp4"))
                .insert_header((header::ACCEPT_RANGES, "bytes"))
                .insert_header((
                    header::CONTENT_RANGE,
                    format!("bytes {}-{}/{}", start, end, file_size),
                ))
                .insert_header((header::CONTENT_LENGTH, chunk_size))
                .streaming(stream);
        }
    }

    // 4. Fallback: stream the entire file if no byte-range requested
    let tokio_file = tokio::fs::File::from_std(file);
    let stream = tokio_util::io::ReaderStream::new(tokio_file);

    HttpResponse::Ok()
        .insert_header((header::CONTENT_TYPE, "video/mp4"))
        .insert_header((header::CONTENT_LENGTH, file_size))
        .streaming(stream)
}

/**
 * Generates a directory path for storing the downloaded video and subtitles based on a unique identifier (UUID).
 * The directory is structured as "resources/videos/{uid}/", where {uid} is the string representation of the UUID.
 * This function helps to organize the downloaded files in a way that avoids conflicts and allows for easy retrieval based on the unique identifier.
 * # Arguments
 * * `uid` - A UUID that serves as a unique identifier for the download, ensuring that each download has its own dedicated directory.
 * # Returns
 * * `PathBuf` - A PathBuf representing the directory path where the downloaded video and subtitles will be stored.
 */
fn directory(uid: &str) -> PathBuf {
    PathBuf::from(format!("resources/videos/{}/", uid))
}

/**
 * Retrieves the path to the downloaded video file for a given unique identifier (UUID).
 * The function constructs the path to the video file based on the directory structure defined in the `directory` function and checks if the file exists.
 * It returns a tuple containing the path to the video file as a string and a boolean indicating whether the file exists and is valid.
 */
fn video(uid: &str) -> Result<String, Error> {
    let path = directory(uid).join("output.mp4");
    if path.exists() {
        Ok(path.to_str().unwrap().to_string())
    } else {
        Err(Error::IOError(IOInfo(
            format!("Video file for video with ID {} not found.", uid),
            None,
        )))
    }
}

/**
* Retrieves the path to the subtitle file for a given unique identifier (UUID) and language.
* The function constructs the path to the subtitle file based on the directory structure defined in the `directory` function and checks if the file exists.
* It returns a tuple containing the path to the subtitle file as a string and a boolean indicating whether the file exists and is valid.
*/
fn subtitle(_context: &Context, uid: &str, lang: &str) -> Result<String, Error> {
    let path = directory(uid).join(format!("output.{}.vtt", lang));
    if path.exists() {
        Ok(path.to_str().unwrap().to_string())
    } else {
        Err(Error::IOError(IOInfo(
            format!(
                "Subtitle file for video {} in language {} not found.",
                uid, lang
            ),
            None,
        )))
    }
}
