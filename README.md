# mdbook-chapter-zero

A preprocessor for [mdBook](https://github.com/rust-lang-nursery/mdBook) which
allows selected (sub-)chapter numbers to begin at 0 instead of 1.

Sometimes you want to have a comprehensive preface with nested subchapters 
or introductory subchapters in specific chapters in your book. 
The prefix chapter from mdbook is just not flexible enough to handle this. 
This preprocessor allows you to have possibly nested (sub-)chapter 0.

## Installation

This preprocessor can be installed with:
```bash
cargo install mdbook-chapter-zero
```

Afterwards, add it to your `book.toml`:
```toml
[preprocessor.chapter-zero]
```

Finally, run `mdbook build` as usual.

## Configuration
This preprocessor accepts two configuration options, with their defaults 
shown below:
```toml
[preprocessor.chapter-zero]
levels = []
marker = "<!-- ch0 -->\n"
```
To apply 0 indexing to all (sub-)chapters at a specific level, use the
`levels` option:
- `levels`: A list of chapter levels which should start at 0 globally.
    Defaults to `[]`, which means no global changes to chapter numbering.
    Here are some examples:
    - If set to `[0]`, then the top level chapters will be 0 indexed.
    - If set to `[1]`, then the first level of subchapters of **all** chapters
        will be 0 indexed.
    - If set to `[0, 1]`, then the top level chapters and the first level of
        subchapters of **all** chapters will be 0 indexed.
    
All (sub-)chapters affected by `levels` will ignore the `marker`.

To apply 0 indexing only to specific (sub-)chapters, use the `marker` option:
- `marker`: A string which signifies that the direct children of this chapter
    should be 0 indexed. Defaults to `<!-- ch0 -->\n`.

To apply 0 indexing to children of a specific (sub-)chapter, add the `marker`
anywhere in the Markdown file of the parent chapter. It is better to have it at 
the top of the file for clarity though. 

You can use any string as the `marker`, not only HTML comments, since the
marker will be removed from the Markdown by the preprocessor. 

### Usage with numbering preprocessors
When used together with processors like 
[mdbook-numeq](https://github.com/yannickseurin/mdbook-numeq)
or
[mdbook-numthm](https://github.com/yannickseurin/mdbook-numthm),
which can number items with section number as prefixes, make sure to ensure
this preprocessor runs before the numbering preprocessors by including e.g.
```toml
[preprocessor.chapter-zero]
before = ["numeq", "numthm"]
```
to your `book.toml`.
