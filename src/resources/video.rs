/**
 * This module provides functionality for downloading videos from a given URL, 
 * extracting subtitles in specified languages, and serving the video and subtitle files through HTTP endpoints.
 */

 // Standard library imports for file handling, process execution, and path manipulation.
use std::path::PathBuf;
use std::process::Command;
use std::io::SeekFrom;
use std::io::Seek;

// Actix-web imports for building the REST API endpoints.
use actix_web::{
    http::header,
    HttpResponse, 
    HttpRequest, 
    Responder
};

// Local module imports for handling video processing and errors.
use tokio::io::AsyncReadExt;

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
pub fn subtitles(uid: uuid::Uuid, lang: &str) -> Result<String, VideoError> {
    match subtitle(uid, lang) {
        Ok(path) => std::fs::read_to_string(path).map_err(|e| VideoError { message: format!("Failed to read subtitle file: {}", e) }),
        Err(e) => Err(e),
    }
}

/**
 * Generates an HTML view for a video with synchronized subtitle playback.
 * The function retrieves the subtitle file for the specified video and language, 
 * parses it using the local WebVTT parser, and generates an HTML page that includes a video player with controls and a track for subtitles.
 * The generated HTML also includes a timeline of buttons for each subtitle cue, allowing users to jump to specific timestamps in the video.
 * When a button is clicked, it triggers a JavaScript function that jumps to the corresponding time in the video, enabling instant subtitle playback synchronization.
 * 
 * Arguments
 * * `uid` - A UUID that serves as a unique identifier for the video resource,
 * * `lang` - A string slice that holds the language code for the subtitles to be displayed in the view.
 * Returns
 * * `Result<String, VideoError>` - A result containing the generated HTML view as a string if successful, 
 * * or a VideoError if there is an error in retrieving the subtitle file, parsing
 * * the VTT content, or any other issues that may arise during the view generation process.
 */
pub fn view(uid: uuid::Uuid, lang: &str) -> Result<String, VideoError> {
    let file = subtitle(uid, lang)?;
    let content = std::fs::read_to_string(file)?;
    let cues = parse_vtt_cues(&content);
  

    // Generate HTML buttons for each subtitle cue, allowing users to jump to specific timestamps in the video. 
    // Each button displays the timestamp and the subtitle text, and when clicked, 
    // it triggers a JavaScript function to jump to the corresponding time in the video.
    let mut buttons = String::new();
    cues.iter().for_each(|cue| {
        let start_seconds = cue.start_seconds;
        let label_minutes = (start_seconds / 60.0).floor() as u64;
        let label_seconds = (start_seconds % 60.0).floor() as u64;
        let time_label = format!("{:02}:{:02}", label_minutes, label_seconds);
        buttons.push_str(&format!(
            r#"<button onclick="jump_to({})">[{}] "{}"</button>"#,
            start_seconds, time_label, cue.text
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
pub fn stream(req: HttpRequest, uid: uuid::Uuid) -> impl Responder {
    let file_info = match video(uid) {
        Ok(info) => info,
        Err(_) => return HttpResponse::NotFound().body(format!("Video with ID {} is not available.", uid)),
    };

    // 1. Open the file synchronously as initiated in your snippet
    let mut file = match std::fs::File::open(&file_info) {
        Ok(f) => f,
        Err(_) => return HttpResponse::InternalServerError().body(format!("Failed to open video file for video with ID {}!!", uid)),
    };

    // 2. Get the full size of the file
    let metadata = match file.metadata() {
        Ok(m) => m,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to read video metadata."),
    };
    let file_size = metadata.len();

    // 3. Parse the standard browser HTTP Range header
    let range_header = req.headers().get(header::RANGE).and_then(|v| v.to_str().ok());

    if let Some(range_str) = range_header {
        if let Some(range) = range_str.strip_prefix("bytes=") {
            let parts: Vec<&str> = range.split('-').collect();
            let start = parts.get(0).and_then(|&s| s.parse::<u64>().ok()).unwrap_or(0);
            let end = parts.get(1)
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
                return HttpResponse::InternalServerError().body("Failed to seek file payload offset.");
            }

            // Convert standard file to a non-blocking streaming chunk payload
            let tokio_file = tokio::fs::File::from_std(file);
            let stream = tokio_util::io::ReaderStream::new(tokio_file.take(chunk_size));

            return HttpResponse::PartialContent()
                .insert_header((header::CONTENT_TYPE, "video/mp4"))
                .insert_header((header::ACCEPT_RANGES, "bytes"))
                .insert_header((header::CONTENT_RANGE, format!("bytes {}-{}/{}", start, end, file_size)))
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
 * The Video struct represents a video resource that has been downloaded from a given URL. 
 * It contains fields for the URL of the video, the path to the downloaded video file,
 * the paths to the extracted subtitles, and the list of languages for which subtitles were extracted. 
 * The video field is a tuple containing the path to the downloaded video file and a boolean indicating whether the file exists and is valid. 
 * The subtitles field is a vector of tuples, each containing the language, path to the subtitle file, and a boolean indicating whether the file exists and is valid. 
 * The languages field is a vector of strings representing the languages for which subtitles were extracted
 */
pub struct Video {
    pub url: String,
    pub video: (String, bool), // (path, is_valid)
    pub subtitles: Vec<(String, String, bool)>, // (language, path, is_valid)
    pub languages: Vec<String>,
}

/**
 * Custom error type for video downloading and subtitle extraction errors. 
 * This struct contains a message field that provides details about the error that occurred during the video processing. 
 * The VideoError struct can be used to represent various types of errors, 
 * such as failures in executing the yt-dlp command, issues with file paths, or problems with subtitle extraction. By using a custom error type, we can provide more specific and informative error messages to help diagnose and resolve issues that may arise during the video processing workflow.
 */
#[derive(Debug)]
pub struct VideoError {
    pub message: String,
}

/**
 * Implements the From trait to convert a standard I/O error into a VideoError.
 */
impl From<std::io::Error> for VideoError {
    fn from(error: std::io::Error) -> Self {
        VideoError { message: format!("IO error: {}", error) }
    }
}

/**
 * Downloads a video from the given URL and extracts subtitles in the specified languages. 
 * It uses the yt-dlp command-line tool to perform the download and subtitle extraction. 
 * The function generates a unique identifier for each download to avoid conflicts and organizes the downloaded files in a structured directory. 
 * The resulting Video struct contains the URL, paths to the downloaded video and subtitles, and the list of languages for which subtitles were extracted.
 * # Arguments
 * * `url` - A string slice that holds the URL of the video to be downloaded.
 * * `languages` - A slice of string slices that holds the languages for which subtitles should be extracted.
 * # Returns
 * * `Result<Video, VideoError>` - A result containing the Video struct if the download and subtitle extraction are successful, 
 * * or a VideoError if any step of the process fails.
 */
pub fn download(url: &str, languages: &[&str]) -> Result<Video, VideoError> {

    // Generate a unique identifier for the download to avoid conflicts and organize files.
    let uid = uuid::Uuid::new_v4();

    // Create the output directory for the downloaded video and subtitles.
    let output = directory(uid);

    println!("Downloading video from URL '{}' to '{}'", url, output.to_str().unwrap());

    // Execute the yt-dlp command to download the video and extract subtitles in the specified languages.
    // https://www.ditig.com/yt-dlp-cheat-sheet#embed-metadata-and-thumbnail
    // yt-dlp -f "mp4" --no-playlist --embed-metadata --embed-thumbnail --movflags +faststart --write-subs --sub-langs "es" "{utl}" -o "output.%(ext)s"
    // We show the progress of the download and subtitle extraction process by printing messages to the console.
    Command::new("yt-dlp")
        .args(&[
            "-f", "mp4",
            "--no-playlist",
            "--embed-metadata",
            "--embed-thumbnail",
            //"--movflags", "+faststart",
            "--write-subs",
            "--sub-langs", &languages.join(","),
            url,
            "-o", output.join("output.%(ext)s").to_str().unwrap(),
        ])
        .stdout(std::process::Stdio::inherit())
        .output()
        .map_err(|e| VideoError { message: format!("Failed to execute yt-dlp: {}", e) })?;

        // Check if the video file was downloaded successfully and if the subtitle files were extracted for the specified languages.
        let video_path = output.join("output.mp4");

        // The video field contains the path to the downloaded video and a boolean indicating whether the file exists and is valid.
        let video = (video_path.to_str().unwrap().to_string(), video_path.exists());

        // The subtitles field contains a vector of tuples, each containing the language, 
        // path to the subtitle file, and a boolean indicating whether the file exists and is valid.
        let subtitles = languages
            .iter()
            .map(|lang| (output.join(format!("output.{}.vtt", lang)), lang))
            .map(|(path, lang)| (lang.to_string(), path.to_str().unwrap().to_string(), path.exists()))
            .collect();

    // The Video struct is returned with the URL, 
    // paths to the downloaded video and subtitles, 
    // and the list of languages for which subtitles were extracted.
    Ok(Video {
        url: url.to_string(),
        languages: languages.iter().map(|&lang| lang.to_string()).collect(),
        video: video,
        subtitles: subtitles,
    })
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
fn directory(uid: uuid::Uuid) -> PathBuf {
    PathBuf::from(format!("resources/videos/{}/", uid))
}

/**
 * Retrieves the path to the downloaded video file for a given unique identifier (UUID). 
 * The function constructs the path to the video file based on the directory structure defined in the `directory` function and checks if the file exists. 
 * It returns a tuple containing the path to the video file as a string and a boolean indicating whether the file exists and is valid. 
 */
fn video(uid: uuid::Uuid) -> Result<String, VideoError> {
    let path = directory(uid).join("output.mp4");
    if path.exists() {
        Ok(path.to_str().unwrap().to_string())
    } else {
        Err(VideoError { message: format!("Video file for video {} not found.", uid) })
    }
}

/**
* Retrieves the path to the subtitle file for a given unique identifier (UUID) and language. 
* The function constructs the path to the subtitle file based on the directory structure defined in the `directory` function and checks if the file exists. 
* It returns a tuple containing the path to the subtitle file as a string and a boolean indicating whether the file exists and is valid. 
*/
fn subtitle(uid: uuid::Uuid, lang: &str) -> Result<String, VideoError> {
    let path = directory(uid).join(format!("output.{}.vtt", lang));
    if path.exists() {
        Ok(path.to_str().unwrap().to_string())
    } else {
        Err(VideoError { message: format!("Subtitle file for video {} in language {} not found.", uid, lang) })
    }
}

/**
 * A minimal cue representation used by the local WebVTT parser.
 */
struct VttCue {
    // Cue start time in seconds from the beginning of the media.
    start_seconds: f64,
    // Cue payload text flattened into a single line for button labels.
    text: String,
}

/**
 * Parses cue timing lines and payload text from raw WebVTT content.
 *
 * Supports cues with optional identifier lines and multi-line payload text.
 */
fn parse_vtt_cues(content: &str) -> Vec<VttCue> {
    let mut cues = Vec::new();
    let lines: Vec<&str> = content.lines().collect();
    let mut i = 0;

    while i < lines.len() {
        let line = lines[i].trim();

        if line.is_empty() || line == "WEBVTT" || line.starts_with("NOTE") {
            i += 1;
            continue;
        }

        // Handle an optional cue identifier by advancing to the following timestamp line.
        if !line.contains(" --> ") {
            if i + 1 < lines.len() && lines[i + 1].contains(" --> ") {
                i += 1;
            } else {
                i += 1;
                continue;
            }
        }

        let timestamp_line = lines[i].trim();
        let Some((start_raw, _)) = timestamp_line.split_once(" --> ") else {
            i += 1;
            continue;
        };

        let Some(start_seconds) = parse_vtt_timestamp_seconds(start_raw.trim()) else {
            i += 1;
            continue;
        };

        i += 1;
        let mut text_lines = Vec::new();
        while i < lines.len() {
            let payload = lines[i].trim();
            if payload.is_empty() {
                break;
            }
            text_lines.push(payload);
            i += 1;
        }

        cues.push(VttCue {
            start_seconds,
            text: text_lines.join(" "),
        });

        while i < lines.len() && lines[i].trim().is_empty() {
            i += 1;
        }
    }

    cues
}

/**
 * Parses a WebVTT timestamp into total seconds.
 *
 * Accepts MM:SS.mmm and HH:MM:SS.mmm formats.
 */
fn parse_vtt_timestamp_seconds(ts: &str) -> Option<f64> {
    let (clock, millis_str) = ts.split_once('.')?;
    let millis = millis_str.parse::<u64>().ok()?;
    let parts: Vec<&str> = clock.split(':').collect();

    let base_seconds = match parts.as_slice() {
        [minutes, seconds] => {
            let minutes = minutes.parse::<u64>().ok()?;
            let seconds = seconds.parse::<u64>().ok()?;
            minutes * 60 + seconds
        }
        [hours, minutes, seconds] => {
            let hours = hours.parse::<u64>().ok()?;
            let minutes = minutes.parse::<u64>().ok()?;
            let seconds = seconds.parse::<u64>().ok()?;
            hours * 3600 + minutes * 60 + seconds
        }
        _ => return None,
    };

    Some(base_seconds as f64 + millis as f64 / 1000.0)
}