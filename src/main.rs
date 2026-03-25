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
    style::{Color, Modifier, Style},
    text::{Line, Span},
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
            // 画面を「縦方向」に3分割するレイアウト
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Percentage(25), // 上の余白（25%）
                    Constraint::Percentage(50), // メインコンテンツ（50%）
                    Constraint::Percentage(25), // 下の余白（25%）
                ])
                .split(f.area());

            let current_slide_text = slides[current_page];

            // 行ごとに処理して「装飾付きの行」のリストを作る
            let mut lines = Vec::new();
            for line in current_slide_text.lines() {
                if line.starts_with("# ") {
                    // 見出し（# ）の場合：記号を消して青・太字にする
                    let header_text = line.trim_start_matches("# ").trim();
                    lines.push(Line::from(Span::styled(
                        header_text,
                        Style::default()
                            .fg(Color::Blue)
                            .add_modifier(Modifier::BOLD),
                    )));
                } else {
                    // 普通の行
                    lines.push(Line::from(line));
                }
            }

            // スライドのウィジェット作成
            let title = format!(" Slide {} / {} ", current_page + 1, slides.len());
            let paragraph = Paragraph::new(lines)
                .block(Block::default().title(title).borders(Borders::ALL))
                .alignment(Alignment::Center);

            f.render_widget(paragraph, chunks[1]);
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
