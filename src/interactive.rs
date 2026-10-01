use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use crate::args::default_excludes;
use crate::errors::write_error_tree;
use crate::scan_errors;

const BANNER: &str = r"
 __      __ ______  ________  ______
|\  \  /  /|\  ___ \|\   __  \|\   _ \
\ \  \/  / | \   __ \ \  \|\  \ \  \\\  \
 \ \    /  | \  \_\ \ \  \\\  \ \  \\\  \
  /     \  | \  _____\ \  \\\  \ \  \\\  \
 /  /\   \ | \ \__/  \ \  _______\\ \_______\
/__/ /\ __\ | \______/  \|_______|\|_______|
|__|/ \|__|               _______
                          |\  ___ \
                          \ \   __/|
                           \ \  \_|/__
                            \ \  \_|\ \
                             \ \_______\
                              \|_______|
";

const TUTORIAL: &str = r#"
================================================================
                          ТУТОРИАЛ
================================================================

Veto — быстрый линтер, который останавливает сборку только на
критических ошибках. Никаких стилистических предупреждений —
только то, что реально ломает код.

Что ищет Veto:
  - Незакрытые Git-конфликты:  <<<<<<<  >>>>>>>  |||||||
  - Критические маркеры в коде:
        TODO_CRITICAL
        FIXME_CRITICAL
        XXX_CRITICAL

Режимы работы:
  1. Просканировать всё    — обходит всю текущую папку
  2. Просканировать папку  — выбираете конкретную папку
  3. Просканировать файл   — выбираете один файл

Результаты:
  Найденные ошибки сохраняются в папке errors/.
  Для каждого проблемного файла создаётся свой JSON-отчёт.
  Например, ошибки в src/main.rs -> errors/src/main.rs.json

Формат отчёта:
  {
    "id": "V004",
    "file": "src/main.rs",
    "line": 42,
    "column": 15,
    "message": "Critical TODO marker present in source.",
    "context": "let x = 10; // TODO_CRITICAL"
  }

Нажмите Enter, чтобы вернуться...
"#;

pub fn run_interactive() -> io::Result<()> {
    loop {
        match main_menu()? {
            MainAction::Tutorial => show_tutorial()?,
            MainAction::Scan => scan_flow()?,
            MainAction::Exit => {
                clear_screen();
                println!("До встречи!");
                return Ok(());
            }
        }
    }
}

enum MainAction {
    Tutorial,
    Scan,
    Exit,
}

enum ScanKind {
    All,
    Folder,
    File,
}

enum Confirm {
    Yes,
    No,
    Cancel,
}

fn main_menu() -> io::Result<MainAction> {
    loop {
        clear_screen();
        println!("{}", BANNER);
        println!("Выберите действие:");
        println!("  » 1. Туториал");
        println!("  » 2. Начать поиск");
        println!("  » 0. Выход");
        print!("\n> ");
        io::stdout().flush()?;

        match read_line()?.as_str() {
            "1" => return Ok(MainAction::Tutorial),
            "2" => return Ok(MainAction::Scan),
            "0" => return Ok(MainAction::Exit),
            _ => {
                println!("\nНеверный выбор. Нажмите Enter, чтобы попробовать снова.");
                pause()?;
            }
        }
    }
}

fn show_tutorial() -> io::Result<()> {
    clear_screen();
    println!("{}", BANNER);
    println!("{}", TUTORIAL);
    pause()
}

fn scan_flow() -> io::Result<()> {
    loop {
        clear_screen();
        println!("{}", BANNER);
        println!("Что сканировать?");
        println!("  » 1. Просканировать всё");
        println!("  » 2. Просканировать папку");
        println!("  » 3. Просканировать файл");
        println!("  » 0. Отмена");
        print!("\n> ");
        io::stdout().flush()?;

        match read_line()?.as_str() {
            "1" => {
                if matches!(confirm("Отлично, сканируем всё?")?, Confirm::Yes) {
                    execute_scan(Path::new("."), ScanKind::All)?;
                    return Ok(());
                }
            }
            "2" => {
                if let Some(folder) = pick_from_list("Выберите папку", &list_subdirs()?)? {
                    let question = format!("Отлично, сканируем '{}'?", folder.display());
                    if matches!(confirm(&question)?, Confirm::Yes) {
                        execute_scan(&folder, ScanKind::Folder)?;
                        return Ok(());
                    }
                }
            }
            "3" => {
                if let Some(file) = pick_from_list("Выберите файл", &list_files()?)? {
                    let question = format!("Отлично, сканируем '{}'?", file.display());
                    if matches!(confirm(&question)?, Confirm::Yes) {
                        execute_scan(&file, ScanKind::File)?;
                        return Ok(());
                    }
                }
            }
            "0" => return Ok(()),
            _ => {
                println!("\nНеверный выбор. Нажмите Enter...");
                pause()?;
            }
        }
    }
}

fn execute_scan(target: &Path, kind: ScanKind) -> io::Result<()> {
    clear_screen();
    println!("{}", BANNER);
    println!("Сканируем '{}'...\n", target.display());

    let excludes = default_excludes();

    let errors = match scan_errors(target, None, &excludes) {
        Ok(found) => found,
        Err(err) => {
            println!("Ошибка сканирования: {}", err);
            pause()?;
            return Ok(());
        }
    };

    let errors_root = Path::new("errors");
    let (strip, prepend): (PathBuf, Option<PathBuf>) = match kind {
        ScanKind::All => (PathBuf::from("."), None),
        ScanKind::Folder => {
            let name = target.file_name().map(PathBuf::from);
            (target.to_path_buf(), name)
        }
        ScanKind::File => {
            let parent = target.parent().unwrap_or(Path::new(".")).to_path_buf();
            (parent, None)
        }
    };

    if let Err(err) = write_error_tree(&errors, errors_root, &strip, prepend.as_deref()) {
        println!("Не удалось записать отчёты: {}", err);
        pause()?;
        return Ok(());
    }

    println!();
    if errors.is_empty() {
        println!("Критических ошибок не найдено.");
        println!("Папка 'errors/' создана пустой.");
    } else {
        println!("Найдено {} критических ошибок.", errors.len());
        println!("Отчёты сохранены в 'errors/'.");
    }

    println!("\nНажмите Enter, чтобы вернуться в главное меню...");
    pause()
}

fn confirm(question: &str) -> io::Result<Confirm> {
    loop {
        clear_screen();
        println!("{}", BANNER);
        println!("{}", question);
        println!("  » 1. Да");
        println!("  » 2. Нет");
        println!("  » 0. Отмена");
        print!("\n> ");
        io::stdout().flush()?;

        match read_line()?.as_str() {
            "1" => return Ok(Confirm::Yes),
            "2" => return Ok(Confirm::No),
            "0" => return Ok(Confirm::Cancel),
            _ => {
                println!("\nНеверный выбор. Нажмите Enter...");
                pause()?;
            }
        }
    }
}

fn pick_from_list(title: &str, items: &[PathBuf]) -> io::Result<Option<PathBuf>> {
    if items.is_empty() {
        clear_screen();
        println!("{}", BANNER);
        println!("{}", title);
        println!("\nЗдесь ничего нет.");
        pause()?;
        return Ok(None);
    }

    loop {
        clear_screen();
        println!("{}", BANNER);
        println!("{}", title);
        println!();

        for (index, item) in items.iter().enumerate() {
            let name = item
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_else(|| item.display().to_string());
            println!("  » {}. {}", index + 1, name);
        }

        println!("  » 0. Назад");
        print!("\n> ");
        io::stdout().flush()?;

        let input = read_line()?;
        let choice: usize = match input.parse() {
            Ok(value) => value,
            Err(_) => {
                println!("\nВведите число.");
                pause()?;
                continue;
            }
        };

        if choice == 0 {
            return Ok(None);
        }

        if let Some(item) = items.get(choice - 1) {
            return Ok(Some(item.clone()));
        }

        println!("\nНеверный выбор. Нажмите Enter...");
        pause()?;
    }
}

fn list_subdirs() -> io::Result<Vec<PathBuf>> {
    let excludes = default_excludes();
    let mut dirs: Vec<PathBuf> = fs::read_dir(".")?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.is_dir())
        .filter(|path| {
            let name = path
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_default();
            !excludes.contains(&name)
        })
        .collect();
    dirs.sort();
    Ok(dirs)
}

fn list_files() -> io::Result<Vec<PathBuf>> {
    let mut files: Vec<PathBuf> = fs::read_dir(".")?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.is_file())
        .filter(|path| path.extension().map(|ext| ext != "json").unwrap_or(true))
        .collect();
    files.sort();
    Ok(files)
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
    let _ = io::stdout().flush();
}

fn read_line() -> io::Result<String> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

fn pause() -> io::Result<()> {
    read_line()?;
    Ok(())
}