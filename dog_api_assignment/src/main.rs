use serde::Deserialize;
use std::error::Error;
use std::fmt;
use std::fs::{create_dir_all, File};
use std::io::copy;
use std::path::{Path, PathBuf};

#[derive(Debug, Deserialize)]
struct DogImage {
    // this is the image link the API gives us
    message: String,

    // the API also returns "success" which we don't really use
    status: String,
}

// my custom error enum to handle different failure cases
#[derive(Debug)]
enum AppError {
    Network(String), // problems connecting or making the request
    Api(String),     // API returns something other than 200
    Parse(String),   // JSON couldn't be turned into the struct
    Io(std::io::Error), // writing files failed
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Network(s) => write!(f, "Network error: {s}"),
            AppError::Api(s) => write!(f, "API error: {s}"),
            AppError::Parse(s) => write!(f, "Parse error: {s}"),
            AppError::Io(e) => write!(f, "IO error: {e}"),
        }
    }
}

impl Error for AppError {}

// makes it easier to convert IO errors into my enum
impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> Self {
        AppError::Io(e)
    }
}

// this function hits the Dog API and returns the data about the image
fn fetch_random_dog_image() -> Result<DogImage, AppError> {
    let url = "https://dog.ceo/api/breeds/image/random";

    // send GET request
    let resp = ureq::get(url)
        .call()
        .map_err(|e| AppError::Network(format!("Request failed: {e}")))?;

    // anything other than 200 is treated as an API-level error
    if resp.status() != 200 {
        return Err(AppError::Api(format!("HTTP {}", resp.status())));
    }

    // ureq v3 splits the response into header + body
    let (_header, mut body) = resp.into_parts();

    // now try to turn the JSON body into a DogImage struct
    body.read_json::<DogImage>()
        .map_err(|e| AppError::Parse(format!("Failed to parse JSON: {e}")))
}

// this actually downloads the image from the URL the API gives
fn download_image(url: &str, dest_path: &Path) -> Result<(), AppError> {
    let resp = ureq::get(url)
        .call()
        .map_err(|e| AppError::Network(format!("Image GET failed: {e}")))?;

    // again make sure the image endpoint actually returned 200
    if resp.status() != 200 {
        return Err(AppError::Api(format!(
            "Image GET HTTP status {}",
            resp.status()
        )));
    }

    // get the image bytes
    let (_header, body) = resp.into_parts();
    let mut reader = body.into_reader();

    // write them to the file system
    let mut out = File::create(dest_path)?;
    copy(&mut reader, &mut out)?;

    Ok(())
}

// creates filenames like dog_1.jpg, dog_2.jpg, etc.
fn image_filename(index: usize) -> PathBuf {
    PathBuf::from(format!("dog_{index}.jpg"))
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Dog Image Downloader");
    println!("====================");
    println!();

    // make a folder called images/ if it doesn't already exist
    let outdir = Path::new("images");
    create_dir_all(outdir)?;

    // assignment wants 5 images
    let count = 5;
    println!("Saving to: {}/", outdir.display());
    println!();

    for i in 1..=count {
        println!("Fetching random dog image #{}", i);

        match fetch_random_dog_image() {
            Ok(record) => {
                println!("Success.");
                println!("Image URL: {}", record.message);

                let dest = outdir.join(image_filename(i));

                // download the actual image file
                match download_image(&record.message, &dest) {
                    Ok(()) => {
                        let size_kb =
                            std::fs::metadata(&dest).map(|m| m.len() / 1024).unwrap_or(0);
                        println!("Saved file: {} ({} KB)", dest.display(), size_kb);
                    }
                    Err(e) => {
                        println!("Download error: {}", e);
                    }
                }
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }

        println!();
    }

    println!("All done. {} images saved.", count);
    Ok(())
}
