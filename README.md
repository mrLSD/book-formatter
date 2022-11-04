# Book formatter
Format text book to html formatted with page separation. Formatting 
with html tag `<p>`, with page counter insertion (calculated by symbols count).


## How it works
1. You should have Text book
2. Book  should separate paragraph as new line.
3. Run: `cargo run -- file_name`
4. Result: `index.html`
5. To change page size set: `const PAGE_SIZE: u64 = 5400;`

Page size calculated by word symbol count. 

### LICENSE: MIT
