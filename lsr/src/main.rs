use std::env;
use std::fs;

#[derive(Clone, Copy)]
enum DataSize {
    Bit,
    Byte,
    Kilobyte,
    Megabyte,
    Gigabyte,
    Terabyte,
    Petabyte,
}

impl DataSize {
    fn short(&self) -> &'static str {
        match self {
            DataSize::Bit => "b",
            DataSize::Byte => "B",
            DataSize::Kilobyte => "KB",
            DataSize::Megabyte => "MB",
            DataSize::Gigabyte => "GB",
            DataSize::Terabyte => "TB",
            DataSize::Petabyte => "PB",
        }
    }

    fn long(&self) -> &'static str {
        match self {
            DataSize::Bit => "bit",
            DataSize::Byte => "byte",
            DataSize::Kilobyte => "kilobyte",
            DataSize::Megabyte => "megabyte",
            DataSize::Gigabyte => "gigabyte",
            DataSize::Terabyte => "terabyte",
            DataSize::Petabyte => "petabyte",
        }
    }
}

#[derive(Clone)]
struct EntryInfo {
    name: String,
    is_dir: bool,
    size: Option<(u64, DataSize)>,
}

fn format_size(mut size: u64) -> (u64, DataSize) {
    let mut unit = DataSize::Byte;
    while size >= 1024 {
        unit = match unit {
            DataSize::Byte => DataSize::Kilobyte,
            DataSize::Kilobyte => DataSize::Megabyte,
            DataSize::Megabyte => DataSize::Gigabyte,
            DataSize::Gigabyte => DataSize::Terabyte,
            DataSize::Terabyte => DataSize::Petabyte,
            _ => break,
        };
        size /= 1024;
    }
    (size, unit)
}

fn read_dir(dir: &str, show_all: bool, show_size: bool) -> Vec<EntryInfo> {
    let Ok(entries) = fs::read_dir(dir) else { return vec![] };

    let mut items: Vec<EntryInfo> = entries
        .flatten()
        .filter_map(|e| {
            let path = e.path();
            let name = path.file_name()?.to_string_lossy().to_string();

            if !show_all && name.starts_with('.') {
                return None;
            }

            let is_dir = e.file_type().ok()?.is_dir();

            let size = if is_dir || !show_size {
                None
            } else {
                e.metadata().ok().map(|m| format_size(m.len()))
            };

            Some(EntryInfo { name, is_dir, size })
        })
        .collect();

    items.sort_by(|a, b| match (a.is_dir, b.is_dir) {
        (true, false) => std::cmp::Ordering::Less,
        (false, true) => std::cmp::Ordering::Greater,
        _ => a.name.cmp(&b.name),
    });

    items
}

fn print_tree(dir: &str, prefix: String, show_all: bool, show_size: bool, show_long: bool, depth: usize) {
    if depth == 0 {
        return;
    }

    let items = read_dir(dir, show_all, show_size);

    for (i, item) in items.iter().enumerate() {
        let last = i == items.len() - 1;
        let conn = if last { "└── " } else { "├── " };

        let size = if show_size {
            if let Some((s, u)) = &item.size {
                let unit = if show_long { u.long() } else { u.short() };
                format!("{} {}", s, unit)
            } else {
                "-".to_string()
            }
        } else {
            String::new()
        };

        println!("{}{}{} {}", prefix, conn, size, item.name);

        if item.is_dir {
            let next = format!("{}/{}", dir, item.name);
            let new_prefix = format!("{}{}", prefix, if last { "    " } else { "│   " });

            print_tree(&next, new_prefix, show_all, show_size, show_long, depth - 1);
        }
    }
}

fn print_flat(items: &[EntryInfo], show_size: bool, show_long: bool) {
    let width = items
        .iter()
        .map(|i| {
            i.size
                .map(|(s, u)| {
                    let ul = if show_long { u.long().len() } else { u.short().len() };
                    format!("{} {}", s, "x".repeat(ul)).len()
                })
                .unwrap_or(1)
        })
        .max()
        .unwrap_or(1);

    for i in items {
        let size = if show_size {
            if let Some((s, u)) = i.size {
                let unit = if show_long { u.long() } else { u.short() };
                format!("{} {}", s, unit)
            } else {
                "-".to_string()
            }
        } else {
            String::new()
        };

        print!("{:>width$} | ", size, width = width);

        if i.is_dir {
            println!("{}/", i.name);
        } else {
            println!("{}", i.name);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut show_all = false;
    let mut show_size = false;
    let mut show_long = false;
    let mut show_inside = false;
    let mut dir = ".";

    for a in &args[1..] {
        match a.as_str() {
            "a" | "all" => show_all = true,
            "s" | "size" => show_size = true,
            "l" | "long" => {
                show_size = true;
                show_long = true;
            }
            "i" | "inside" => show_inside = true,
            _ => dir = a,
        }
    }

    let items = read_dir(dir, show_all, show_size);

    if show_inside {
        println!("{}", dir);
        print_tree(dir, String::new(), show_all, show_size, show_long, 2);
    } else {
        print_flat(&items, show_size, show_long);
    }
}
