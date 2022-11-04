const PAGE_SIZE: u64 = 5400; // 8200 for 12px
const START_HTML: &str = r#"<html><body><div style="font-family: verdana;font-size: 60px; font-weight: bold; text-align: justify">"#;
const END_HTML: &str = r#"</div></body></html>"#;

fn main() -> std::io::Result<()> {
    let args = std::env::args();
    let a: Vec<String> = args.collect();
    if a.len() < 2 {
        eprintln!("Set input file");
        return Ok(());
    }
    let data = std::fs::read_to_string(&a[1])?;

    let mut html: Vec<String> = vec![START_HTML.to_string()];
    let mut page_counter: u64 = 0;
    let mut page = 0;
    let mut page_words: Vec<&str> = vec![];
    for p in data.split("\n").collect::<Vec<&str>>() {
        // New paragraph
        page_words.push("\n<p>");

        for word in p.split(" ").collect::<Vec<&str>>() {
            if (page_counter + word.len() as u64) < PAGE_SIZE {
                page_counter += word.len() as u64;
                page_words.push(word);
            } else {
                page += 1;
                page_counter = 0;
                let words = page_words.join(" ");
                html.push(
                    [
                        "<p>",
                        &words,
                        "</p>",
                        "\n<p>[-",
                        &page.to_string(),
                        "-]</p>",
                    ]
                    .join(""),
                );
                page_words = vec![word];
            }
        }
    }

    // last page
    page += 1;
    let words = page_words.join(" ");
    html.push(
        [
            "<p>",
            &words,
            "</p>",
            "\n<p>[-",
            &page.to_string(),
            "-]</p>",
        ]
        .join(""),
    );

    html.push(END_HTML.to_string());
    std::fs::write("index.html", html.join("\n"))
}
