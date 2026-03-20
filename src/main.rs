use std::env;
use std::fs;
use std::io;

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::Alignment,
    widgets::{Block, Borders, Paragraph},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("エラー: ファイルパスを指定してください。");
        std::process::exit(1);
    }

    let file_path = &args[1];
    let content = fs::read_to_string(file_path)?;

    let slides: Vec<&str> = content.split("---").map(|s| s.trim()).collect();
    if slides.is_empty() {
        eprintln!("スライドが空です。");
        std::process::exit(1);
    }

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // 描画処理
    terminal.draw(|f| {
        let paragraph = Paragraph::new(slides[0])
            .block(Block::default().title(" Slide 1 ").borders(Borders::ALL))
            .alignment(Alignment::Center);

        f.render_widget(paragraph, f.area());
    })?;

    // キー入力待ちの無限ループ
    loop {
        if let Event::Key(key) = event::read()? {
            if key.code == KeyCode::Char('q') {
                break;
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}
