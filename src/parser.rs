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
        let mut content = parse_inline_text(text);
        let mut spans = vec![bullet];
        spans.append(&mut content);
        Line::from(spans)
    } else if line.starts_with("> ") {
        let text = line.trim_start_matches("> ").trim();
        // 引用符として左側に緑色の縦線を入れ、文字を斜体(Italic)で暗くする
        let quote_mark = Span::styled("┃ ", Style::default().fg(Color::Green));
        let content = Span::styled(
            text,
            Style::default()
                .fg(Color::Gray)
                .add_modifier(Modifier::ITALIC),
        );
        Line::from(vec![quote_mark, content])
    } else {
        Line::from(parse_inline_text(line))
    }
}

// 文字列を "**" で分割して、通常文字と太字を交互にSpanにする関数
fn parse_inline_text<'a>(text: &'a str) -> Vec<Span<'a>> {
    let mut spans = Vec::new();
    // "**" で文字列を分割する
    let parts: Vec<&str> = text.split("**").collect();

    for (i, part) in parts.iter().enumerate() {
        if i % 2 == 1 {
            // 奇数番目（**で囲まれた内側）は太字にする
            spans.push(Span::styled(
                *part,
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(Color::White),
            ));
        } else {
            // 偶数番目（外側）は通常のテキスト
            spans.push(Span::raw(*part));
        }
    }
    spans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_inline_text() {
        let input = "通常文字と**太字**のテスト";

        let result = parse_inline_text(input);

        // "通常文字と"、"太字"、"のテスト" の3つのパーツに分かれているはず
        assert_eq!(result.len(), 3);

        // 中身のテキストが合っているか確認
        assert_eq!(result[0].content, "通常文字と");
        assert_eq!(result[1].content, "太字");
        assert_eq!(result[2].content, "のテスト");

        // 2番目のパーツが本当に太字になっているか確認
        assert!(result[1].style.add_modifier.contains(Modifier::BOLD));
    }
}
