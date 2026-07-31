use anyhow::{Context, Result};
use chrono::NaiveDate;
use percent_encoding::{NON_ALPHANUMERIC, utf8_percent_encode};
use serde::{Deserialize, Serialize};
use std::{
    fmt::Write,
    fs::{self, DirEntry},
    path::PathBuf,
    process::Command,
};

#[derive(Serialize, Deserialize, Debug)]
struct BlogEntryInfo {
    published: NaiveDate,
    title: String,
}

struct BlogEntry {
    info: BlogEntryInfo,
    directory: PathBuf,
}

fn build_blog_entry(dir: DirEntry) -> Result<BlogEntry> {
    let mut info_path = dir.path();

    info_path.push("info.json");

    let info =
        fs::read_to_string(&info_path).with_context(|| format!("failed read {info_path:?}"))?;

    let info: BlogEntryInfo =
        serde_json::from_str(&info).with_context(|| format!("failed parse {info}"))?;

    Ok(BlogEntry {
        info,
        directory: dir.path(),
    })
}

fn main() -> Result<()> {
    eprintln!(
        "example blog entry data structure: {}",
        serde_json::to_string(&BlogEntryInfo {
            title: "example title".to_string(),
            published: NaiveDate::from_ymd_opt(2026, 7, 30).unwrap(),
        })?
    );

    eprintln!("Building website");

    fs::create_dir_all("output")?;
    fs::copy("index.html", "output/index.html")?;
    fs::copy("style.css", "output/style.css")?;
    fs::create_dir_all("output/blog")?;

    eprintln!("reading blog directory");

    let pages: fs::ReadDir = fs::read_dir("./blog/")?;

    let mut links = String::new();

    eprintln!("enumerating pages");

    let mut pages = pages
        .filter_map(|page| match page {
            Ok(page) => match build_blog_entry(page) {
                Ok(page) => Some(page),
                Err(err) => {
                    eprintln!("skipping because of: {:?}", err);
                    None
                }
            },
            Err(_err) => None,
        })
        .collect::<Vec<_>>();

    pages.sort_by_key(|page| page.info.published);

    for page in pages {
        eprintln!("generating page: {:?}", page.directory);

        let mut main_file = page.directory.clone();

        main_file.push("main.typ");

        let mut output = PathBuf::from("output/blog");

        output.push(
            page.directory
                .file_name()
                .context("failed to get file name of directory")?,
        );

        fs::create_dir_all(&output)?;

        for file in fs::read_dir(&page.directory)? {
            let path = file?.path();

            let file_name = path.file_name().context("failed to get file name")?;

            let mut output_path = output.clone();

            output_path.push(file_name);

            fs::copy(path, output_path)?;
        }

        output.push("index.html");

        Command::new("typst")
            .args([
                "compile",
                main_file
                    .to_str()
                    .with_context(|| format!("failed to make {main_file:?} a str"))?,
                "--format",
                "html",
                "--features",
                "html",
                "--input",
                format!("title={}", page.info.title).as_str(),
                "--root",
                ".",
                dbg!(&output)
                    .to_str()
                    .with_context(|| format!("failed to make {output:?} a str"))?,
            ])
            .status()
            .context("failed to invoke typst")?;

        let page_link = page
            .directory
            .file_name()
            .context("failed to convert directory name to string")?
            .to_str()
            .context("failed to convert directory name from ostr to str")?;

        write!(
            &mut links,
            "<a href=\"/blog/{}/\">{}</a>",
            utf8_percent_encode(page_link, NON_ALPHANUMERIC),
            page.info.title,
        )?
    }

    let blog_page = format!(
        "
<!DOCTYPE html>\
<html lang=\"en\">\
  <head>\
    <meta charset=\"UTF-8\">\
    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\
    <meta http-equiv=\"X-UA-Compatible\" content=\"ie=edge\">\
    <title>My Website</title>\
    <link rel=\"stylesheet\" href=\"../style.css\">\
    <link rel=\"icon\" href=\"./favicon.ico\" type=\"image/x-icon\">\
  </head>\
  <body>\
    <nav class=\"navbar\">\
        <a href=\"/\">Home</a>\
        <a href=\"/blog\">Blog</a>\
    </nav>\
    <div class=\"content\">\
      <h1>Posts</h1>\
      {links}\
    </div>\
  </body>\
</html>
"
    );

    eprintln!("writing output/blog/index.html output");

    fs::write("output/blog/index.html", blog_page).context("writing blog index page")?;

    Ok(())
}
