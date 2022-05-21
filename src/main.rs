use fltk::{app, prelude::*, window::Window, text::TextEditor, text::TextBuffer};
use std::fs::File;
use std::env;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        match args[1].to_lowercase().as_str() {
            "--help" => help(),
            "-v" | "--version" => version(),
            _ => build_app(open_or_create_file(&args[1])),
        }
    } else {
        build_app(TextBuffer::default());
    }
}

fn open_or_create_file(file_path: &String) -> TextBuffer {
    /*        Open / Create File      */
    let mut file;
    let mut buff = TextBuffer::default();
    let open_file = File::open(file_path);
    match open_file {
        Err(_)      => {
            File::create(file_path).expect(format!("Failed to open/create file at {}", file_path).as_str());
            file = File::open(file_path).unwrap();
        }
        Ok(cont)    => file = cont,
    }
    let mut file_content = String::new();
    file.read_to_string(&mut file_content).unwrap();
    buff.set_text(&file_content);
    buff
}

fn build_app(buff: TextBuffer) {
    let app = app::App::default();
    let mut wind = Window::new(100, 100, 400, 300, "Eranite");
    let mut editor = TextEditor::new(0, 0, 200, 200, "Editor");
    editor.set_buffer(Some(buff));
    wind.make_resizable(true);
    wind.end();
    wind.show();
    app.run().unwrap();
}


fn help() {
    println!(
        "Options:
        --help           Show help
        --version    -v  Show program version");
    }

fn version() {
    println!("version: 0.1.0");
}
