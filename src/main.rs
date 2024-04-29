const PAGE_SIZE: u64 = 2700; // 8200 for 12px
const START_HTML: &str = r#"<html><head><meta charset="UTF-8"><.head><body><div style="font-family: verdana;font-size: 60px; font-weight: bold; text-align: left">"#;
const END_HTML: &str = r#"</div></body></html>"#;

fn main() -> std::io::Result<()> {
    let args = std::env::args();
    let a: Vec<String> = args.collect();
    if a.len() < 2 {
        eprintln!("Set input file");
        return Ok(());
    }
    let data = std::fs::read_to_string(&a[1])?;
    let pages_per_file = if a.len() >= 3 {
        a[2].parse::<u64>()
            .expect("expected page size per file as integer")
    } else {
        0
    };
    let result_file = if a.len() == 4 {
        a[3].clone()
    } else {
        "index".to_string()
    };

    let mut html: Vec<String> = vec![START_HTML.to_string()];
    let mut page_counter: u64 = 0;
    let mut page = 0;
    let mut page_words: Vec<&str> = vec![];
    for p in data.split('\n').collect::<Vec<&str>>() {
        // New paragraph
        page_words.push("\n<p>");

        for word in p.split(' ').collect::<Vec<&str>>() {
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
                if pages_per_file > 0 {
                    if page % pages_per_file == 0 {
                        let file_index = page / pages_per_file;
                        let file_name = format!("{result_file}{file_index}.html");
                        html.push(END_HTML.to_string());
                        std::fs::write(file_name, html.join("\n")).unwrap();
                        // Re-init html
                        html = vec![START_HTML.to_string()];
                    }
                }
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
