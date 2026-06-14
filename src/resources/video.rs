
/**
 * This module defines the Video struct, which represents a video resource that has been downloaded from a given URL. 
 * The Video struct contains fields for the URL of the video, the path to the downloaded video file,
 * the paths to the extracted subtitles, and the list of languages for which subtitles were extracted.
 */
use std::path::PathBuf;
use std::process::Command;

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
    let output_dir = PathBuf::from(format!("resources/videos/{}/", uid));

    println!("Downloading video from URL: {}", url);

    // Execute the yt-dlp command to download the video and extract subtitles in the specified languages.
    // https://www.ditig.com/yt-dlp-cheat-sheet#embed-metadata-and-thumbnail
    // yt-dlp -f "mp4" --no-playlist --embed-metadata --embed-thumbnail --write-subs --sub-langs "es" "{utl}" -o "output.%(ext)s"
    // We show the progress of the download and subtitle extraction process by printing messages to the console.
    Command::new("yt-dlp")
        .args(&[
            "-f", "mp4",
            "--no-playlist",
            "--embed-metadata",
            "--embed-thumbnail",
            "--write-subs",
            "--sub-langs", &languages.join(","),
            url,
            "-o", output_dir.join("output.%(ext)s").to_str().unwrap(),
        ])
        .stdout(std::process::Stdio::inherit())
        .output()
        .map_err(|e| VideoError { message: format!("Failed to execute yt-dlp: {}", e) })?;

        // Check if the video file was downloaded successfully and if the subtitle files were extracted for the specified languages.
        let video_path = output_dir.join("output.mp4");

        // The video field contains the path to the downloaded video and a boolean indicating whether the file exists and is valid.
        let video = (video_path.to_str().unwrap().to_string(), video_path.exists());

        // The subtitles field contains a vector of tuples, each containing the language, 
        // path to the subtitle file, and a boolean indicating whether the file exists and is valid.
        let subtitles = languages
            .iter()
            .map(|lang| (output_dir.join(format!("output.{}.vtt", lang)), lang))
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