use mdbook::book::Book;
use mdbook::errors::{Error, Result};
use mdbook::preprocess::{Preprocessor, PreprocessorContext};
use mdbook::BookItem;
use std::convert::{TryFrom, TryInto};
use toml::value::Table;

static DEFAULT_MARKER: &str = "<!-- ch0 -->\n";

pub struct ChapterZeroPreprocessor;

/// Configuration for Table of Contents generation
#[derive(Debug)]
pub struct Config {
    /// Levels for which chapter zero should be applied globally.
    /// Defaults to [], which does not apply any global changes.
    /// If set to [0], then the top level chapters will be 0 indexed.
    /// If set to [1], then the first level of subchapters of ALL chapters
    /// will be 0 indexed.
    /// If set to [0, 1], then the top level chapters and the first level of
    /// subchapters of ALL chapters will be 0 indexed.
    /// All (sub-)chapters affected by this setting will ignore the `marker`.
    pub levels: Vec<usize>,
    /// Marker to signify that the direct children of this chapter should be 0 indexed.
    /// Defaults to `<!-- ch0 -->\n`.
    pub marker: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            levels: vec![],
            marker: DEFAULT_MARKER.to_string(),
        }
    }
}

impl<'a> TryFrom<Option<&'a Table>> for Config {
    type Error = Error;

    fn try_from(mdbook_cfg: Option<&Table>) -> Result<Config> {
        let mut cfg = Config::default();
        let mdbook_cfg = match mdbook_cfg {
            Some(c) => c,
            None => return Ok(cfg),
        };

        if let Some(levels) = mdbook_cfg.get("levels") {
            let levels_array = match levels.as_array() {
                Some(array) => array,
                None => {
                    return Err(Error::msg(format!(
                        "Levels {levels:?} is not a valid array"
                    )))
                }
            };

            let mut levels: Vec<usize> = Vec::new();
            for level_val in levels_array {
                match level_val.as_integer() {
                    Some(level) if level >= 0 => levels.push(level as usize),
                    _ => {
                        return Err(Error::msg(format!(
                            "Level {level_val} is not a valid usize"
                        )))
                    }
                };
            }

            cfg.levels = levels.try_into()?;
        }

        if let Some(marker) = mdbook_cfg.get("marker") {
            let marker = match marker.as_str() {
                Some(m) => m,
                None => {
                    return Err(Error::msg(format!(
                        "Marker {marker:?} is not a valid string"
                    )))
                }
            };
            cfg.marker = marker.into();
        }

        Ok(cfg)
    }
}

impl ChapterZeroPreprocessor {
    pub fn new() -> ChapterZeroPreprocessor {
        ChapterZeroPreprocessor
    }
}
impl Preprocessor for ChapterZeroPreprocessor {
    fn name(&self) -> &str {
        "chapter-zero"
    }

    fn run(&self, ctx: &PreprocessorContext, mut book: Book) -> Result<Book, Error> {
        let cfg: Config = ctx.config.get_preprocessor(self.name()).try_into()?;
        log::debug!("Config: {cfg:?}");

        let mut local_ch0_vec: Vec<Vec<u32>> = Vec::new();

        book.for_each_mut(|item: &mut BookItem| {
            if let BookItem::Chapter(chapter) = item {
                match chapter.number.as_mut() {
                    Some(sn) => {
                        // Handle global chapter zero
                        let chapter_levels = (0..sn.0.len()).collect::<Vec<usize>>();
                        for chl in &chapter_levels {
                            if cfg.levels.contains(chl) {
                                sn.0[*chl] -= 1;
                            }
                        }

                        // Save chapters marked for local chapter zero
                        let content = &chapter.content.replace("\r\n", "\n");
                        if content.contains(cfg.marker.as_str()) {
                            if !cfg.levels.contains(&sn.0.len()) {
                                local_ch0_vec.push(sn.0.clone());
                                chapter.content = content.replace(cfg.marker.as_str(), "");
                            }
                        }
                    }
                    None => {}
                }
            }
        });
        log::debug!("Local chapter zero will be applied to: {local_ch0_vec:?}");

        // Apply local chapter zero
        book.for_each_mut(|item: &mut BookItem| {
            if let BookItem::Chapter(chapter) = item {
                match chapter.number.as_mut() {
                    Some(sn) => {
                        for local_ch0_sn in &local_ch0_vec {
                            if sn.0.starts_with(local_ch0_sn) && sn.0.len() > local_ch0_sn.len() {
                                sn.0[local_ch0_sn.len()] -= 1;
                            }
                        }
                    }
                    None => {}
                }
            }
        });

        Ok(book)
    }

    fn supports_renderer(&self, renderer: &str) -> bool {
        renderer != "not-supported"
    }
}
