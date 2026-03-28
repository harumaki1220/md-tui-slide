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
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
};

mod parser;
use parser::parse_markdown_line;

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
            // 画面全体を縦方向に3分割する
            let vertical_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Percentage(20), // 上の余白（少し狭くしました）
                    Constraint::Percentage(60), // スライドの高さ
                    Constraint::Percentage(20), // 下の余白
                ])
                .split(f.area());

            // 縦の真ん中エリアをさらに横方向に3分割する
            let horizontal_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(15), // 左の余白
                    Constraint::Percentage(70), // スライドの横幅
                    Constraint::Percentage(15), // 右の余白
                ])
                .split(vertical_chunks[1]);

            let current_slide_text = slides[current_page];

            let mut lines = Vec::new();
            let mut in_code_block = false;

            for line in current_slide_text.lines() {
                lines.push(parse_markdown_line(line, &mut in_code_block));
            }

            let title = format!(" Slide {} / {} ", current_page + 1, slides.len());
            let paragraph = Paragraph::new(lines)
                .block(Block::default().title(title).borders(Borders::ALL))
                .alignment(Alignment::Left); // 文字自体は読みやすいように左揃え

            // 縦横に分割されたど真ん中のエリア
            f.render_widget(paragraph, horizontal_chunks[1]);

            let footer_text = " q: 終了 | ←/→: ページ移動 ";
            let footer = Paragraph::new(footer_text)
                .style(Style::default().fg(Color::DarkGray))
                .alignment(Alignment::Center);

            // 縦に3分割した一番下のエリアに描画する
            f.render_widget(footer, vertical_chunks[2]);
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
