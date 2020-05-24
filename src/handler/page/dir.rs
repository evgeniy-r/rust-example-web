use std::cmp::Ordering;
use std::path::{Path, PathBuf};

use super::super::response::Body;

#[derive(Eq)]
pub struct DirItem<'a> {
    path: &'a Path,
    is_dir: bool,
}

impl<'a> Ord for DirItem<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        match other.is_dir.cmp(&self.is_dir) {
            Ordering::Equal => self.path.cmp(&other.path),
            o => o,
        }
    }
}

impl<'a> PartialOrd for DirItem<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> PartialEq for DirItem<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path && self.is_dir == other.is_dir
    }
}

impl<'a> DirItem<'a> {
    pub fn from_abs_path(path: &'a PathBuf, root: &PathBuf) -> Option<Self> {
        let is_dir = path.is_dir();
        path.strip_prefix(root)
            .ok()
            .map(|path| Self { path, is_dir })
    }
}

pub fn body(p: &Path, items: Vec<DirItem>) -> Body {
    let title = name(p);
    format!(
        r#"
            <!DOCTYPE html>
            <html>
                <head>
                <meta charset="utf-8" />
                <title>{}</title>
                </head>
                <body style="padding: 50px 20%;">
                    <div>{}</div>
                    <h1>{}</h1>
                    {}
                    {}
                </body>
            </html>
        "#,
        title,
        breadcrumbs(p),
        title,
        up(p),
        list(items)
    )
    .as_bytes()
    .to_vec()
}

fn up(p: &Path) -> String {
    match p.ancestors().nth(1) {
        Some(i) => link("...", i),
        None => String::from(""),
    }
}

fn breadcrumbs(p: &Path) -> String {
    let mut breadcrumbs: Vec<_> = p.ancestors().skip(1).map(|i| link(&name(i), i)).collect();
    breadcrumbs.reverse();
    breadcrumbs.join(" / ")
}

fn list(mut items: Vec<DirItem>) -> String {
    items.sort();
    let items: Vec<_> = items
        .iter()
        .map(|i| {
            let style = if i.is_dir {
                " style=\"font-weight: bold;\""
            } else {
                ""
            };
            format!("<div{}>{}</div>", style, link(&name(i.path), i.path))
        })
        .collect();
    items.join("")
}

fn name(p: &Path) -> String {
    p.file_name()
        .map(|f| f.to_string_lossy().to_string())
        .unwrap_or(String::from("Home"))
}

fn link(name: &str, p: &Path) -> String {
    format!("<a href=\"/{}\">{}</a>", p.to_str().unwrap_or("#"), name)
}
