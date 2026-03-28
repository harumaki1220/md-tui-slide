use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
};

pub fn parse_markdown_line<'a>(line: &'a str, in_code_block: &mut bool) -> Line<'a> {
    // コードブロックの開始/終了を検知（``` で始まる行）
    if line.starts_with("```") {
        *in_code_block = !*in_code_block;
        return Line::from(Span::styled(
            format!("  {:<60}  ", line),
            Style::default().fg(Color::Gray).bg(Color::DarkGray),
        ));
    }

    // コードブロック「内部」の行の装飾
    if *in_code_block {
        return Line::from(Span::styled(
            format!("  {:<60}  ", line), // 見栄えのために左右に空白を入れる
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
