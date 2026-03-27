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

// 1行の文字列を受け取り、装飾された Line を返す
fn parse_markdown_line<'a>(line: &'a str, in_code_block: &mut bool) -> Line<'a> {
    // コードブロックの開始/終了を検知（``` で始まる行）
    if line.starts_with("```") {
        *in_code_block = !*in_code_block; // trueとfalseを反転させる
        return Line::from(Span::styled(line, Style::default().fg(Color::DarkGray)));
    }

    // コードブロック「内部」の行の装飾
    if *in_code_block {
        return Line::from(Span::styled(
            format!("  {}  ", line), // 見栄えのために左右に空白を入れる
            Style::default().fg(Color::Cyan).bg(Color::DarkGray),
        ));
    }

    // それ以外の通常のMarkdown構文（既存のコードをそのまま配置）
    if line.starts_with("# ") {
        let text = line.trim_start_matches("# ").trim();
        Line::from(Span::styled(
            text,
            Style::default()
                .fg(Color::Blue)
                .add_modifier(Modifier::BOLD),
        ))
    } else if line.starts_with("## ") {
        // 見出し2（緑・太字）
        let text = line.trim_start_matches("## ").trim();
        Line::from(Span::styled(
            text,
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        ))
    } else if line.starts_with("- ") {
        // 箇条書き（黄色）
        let text = line.trim_start_matches("- ").trim();
        let bullet = Span::styled("• ", Style::default().fg(Color::Yellow));
        let content = Span::raw(text);
        Line::from(vec![bullet, content])
    } else {
        // 普通のテキスト（白）
        Line::from(line)
    }
}

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
            let mut in_code_block = false;

            for line in current_slide_text.lines() {
                lines.push(parse_markdown_line(line, &mut in_code_block));
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
