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

fn parse_inline_text<'a>(mut text: &'a str) -> Vec<Span<'a>> {
    let mut spans = Vec::new();

    while !text.is_empty() {
        // 次の "**" と "`" がどこにあるか探す
        let bold_idx = text.find("**");
        let code_idx = text.find('`');

        let (next_marker, next_idx) = match (bold_idx, code_idx) {
            (Some(b), Some(c)) if b < c => ("**", b), // 太字が先
            (Some(_), Some(c)) => ("`", c),           // コードが先
            (Some(b), None) => ("**", b),             // 太字しかない
            (None, Some(c)) => ("`", c),              // コードしかない
            (None, None) => {
                spans.push(Span::raw(text));
                break;
            }
        };

        if next_idx > 0 {
            spans.push(Span::raw(&text[..next_idx]));
        }

        let after_marker = &text[next_idx + next_marker.len()..];

        if let Some(end_idx) = after_marker.find(next_marker) {
            let content = &after_marker[..end_idx];

            // 記号の種類に応じて装飾を変える
            if next_marker == "**" {
                spans.push(Span::styled(
                    content,
                    Style::default()
                        .add_modifier(Modifier::BOLD)
                        .fg(Color::White),
                ));
            } else {
                spans.push(Span::styled(
                    content,
                    Style::default().fg(Color::Red).bg(Color::DarkGray), // コードは赤文字＋背景グレー
                ));
            }

            text = &after_marker[end_idx + next_marker.len()..];
        } else {
            spans.push(Span::raw(&text[..next_idx + next_marker.len()]));
            text = after_marker;
        }
    }

    spans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_inline_text() {
        let input = "通常、**太字**、そして `コード` のテスト";

        let result = parse_inline_text(input);

        // "通常、" | "太字" | "、そして " | "コード" | " のテスト" -> 5パーツになるはず
        assert_eq!(result.len(), 5);

        assert_eq!(result[0].content, "通常、");

        assert_eq!(result[1].content, "太字");
        assert!(result[1].style.add_modifier.contains(Modifier::BOLD));

        assert_eq!(result[2].content, "、そして ");

        assert_eq!(result[3].content, "コード");
        assert_eq!(result[3].style.fg, Some(Color::Red)); // コードは赤色になっているか？

        assert_eq!(result[4].content, " のテスト");
    }
}
