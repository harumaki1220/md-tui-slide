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

    let mut current_page = 0;

    loop {
        // 描画処理
        terminal.draw(|f| {
            // current_page 番目のスライドを表示
            let title = format!(" Slide {} / {} ", current_page + 1, slides.len());
            let paragraph = Paragraph::new(slides[current_page])
                .block(Block::default().title(title).borders(Borders::ALL))
                .alignment(Alignment::Center);

            f.render_widget(paragraph, f.area());
        })?;

        // 入力処理
        if let Event::Key(key) = event::read()? {
            match key.code {
                // 'q' で終了
                KeyCode::Char('q') => break,

                // 右矢印キーで次のページへ
                KeyCode::Right => {
                    if current_page < slides.len() - 1 {
                        current_page += 1;
                    }
                }

                // 左矢印キーで前のページへ
                KeyCode::Left => {
                    if current_page > 0 {
                        current_page -= 1;
                    }
                }
                _ => {}
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}
