use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use zed_extension_api as zed;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CommentDividerSettings {
    length: usize,
    should_length_include_indent: bool,
    main_header_filler: String,
    main_header_height: String,
    main_header_align: String,
    main_header_transform: String,
    subheader_filler: String,
    subheader_height: String,
    subheader_align: String,
    subheader_transform: String,
    line_filler: String,
    languages_map: HashMap<String, Vec<String>>,
}

impl Default for CommentDividerSettings {
    fn default() -> Self {
        let mut languages_map = HashMap::new();

        // Default language mappings
        languages_map.insert("javascript".to_string(), vec!["//".to_string()]);
        languages_map.insert("typescript".to_string(), vec!["//".to_string()]);
        languages_map.insert("rust".to_string(), vec!["//".to_string()]);
        languages_map.insert("python".to_string(), vec!["#".to_string()]);
        languages_map.insert(
            "html".to_string(),
            vec!["<!--".to_string(), "-->".to_string()],
        );
        languages_map.insert("css".to_string(), vec!["/*".to_string(), "*/".to_string()]);
        languages_map.insert("c".to_string(), vec!["/*".to_string(), "*/".to_string()]);
        languages_map.insert("cpp".to_string(), vec!["//".to_string()]);
        languages_map.insert("java".to_string(), vec!["//".to_string()]);
        languages_map.insert("go".to_string(), vec!["//".to_string()]);
        languages_map.insert("php".to_string(), vec!["//".to_string()]);
        languages_map.insert("ruby".to_string(), vec!["#".to_string()]);
        languages_map.insert("shell".to_string(), vec!["#".to_string()]);
        languages_map.insert("bash".to_string(), vec!["#".to_string()]);
        languages_map.insert("yaml".to_string(), vec!["#".to_string()]);
        languages_map.insert("toml".to_string(), vec!["#".to_string()]);
        languages_map.insert("sql".to_string(), vec!["--".to_string()]);
        languages_map.insert("lua".to_string(), vec!["--".to_string()]);
        languages_map.insert("vim".to_string(), vec!["\"".to_string()]);

        Self {
            length: 80,
            should_length_include_indent: false,
            main_header_filler: "-".to_string(),
            main_header_height: "block".to_string(),
            main_header_align: "center".to_string(),
            main_header_transform: "none".to_string(),
            subheader_filler: "-".to_string(),
            subheader_height: "line".to_string(),
            subheader_align: "center".to_string(),
            subheader_transform: "none".to_string(),
            line_filler: "-".to_string(),
            languages_map,
        }
    }
}

struct CommentDividerExtension {
    settings: CommentDividerSettings,
}

impl CommentDividerExtension {
    fn new() -> Self {
        Self {
            settings: CommentDividerSettings::default(),
        }
    }

    fn get_comment_chars(&self, language: &str) -> Option<&Vec<String>> {
        self.settings.languages_map.get(language)
    }

    fn transform_text(&self, text: &str, transform: &str) -> String {
        match transform {
            "uppercase" => text.to_uppercase(),
            "lowercase" => text.to_lowercase(),
            "capitalize" => {
                let mut chars = text.chars();
                match chars.next() {
                    None => String::new(),
                    Some(first) => {
                        first.to_uppercase().collect::<String>() + &chars.as_str().to_lowercase()
                    }
                }
            }
            _ => text.to_string(),
        }
    }

    fn align_text(&self, text: &str, total_width: usize, align: &str, filler: &str) -> String {
        let text_len = text.len();
        if text_len >= total_width {
            return text.to_string();
        }

        let remaining = total_width - text_len;

        match align {
            "left" => format!("{}{}", text, filler.repeat(remaining)),
            "right" => format!("{}{}", filler.repeat(remaining), text),
            "center" => {
                let left_padding = remaining / 2;
                let right_padding = remaining - left_padding;
                format!(
                    "{}{}{}",
                    filler.repeat(left_padding),
                    text,
                    filler.repeat(right_padding)
                )
            }
            _ => text.to_string(),
        }
    }

    fn get_effective_length(&self, indent_size: usize) -> usize {
        if self.settings.should_length_include_indent {
            self.settings.length.saturating_sub(indent_size)
        } else {
            self.settings.length
        }
    }

    fn make_main_header(&self, text: &str, language: &str, indent_size: usize) -> String {
        let comment_chars = self.get_comment_chars(language);
        if comment_chars.is_none() {
            return format!("// Unsupported language: {}", language);
        }

        let chars = comment_chars.unwrap();
        let start_char = &chars[0];
        let end_char = if chars.len() > 1 {
            &chars[1]
        } else {
            start_char
        };

        let effective_length = self.get_effective_length(indent_size);
        let transformed_text = self.transform_text(text, &self.settings.main_header_transform);

        let indent = " ".repeat(indent_size);

        if self.settings.main_header_height == "block" {
            // Three-line block header
            let filler_line = format!(
                "{}{} {} {}",
                indent,
                start_char,
                self.settings
                    .main_header_filler
                    .repeat(effective_length - start_char.len() - end_char.len() - 2),
                end_char
            );

            let text_content_width = effective_length - start_char.len() - end_char.len() - 2;
            let aligned_text = self.align_text(
                &transformed_text,
                text_content_width,
                &self.settings.main_header_align,
                " ",
            );
            let text_line = format!("{}{} {} {}", indent, start_char, aligned_text, end_char);

            format!("{}\n{}\n{}", filler_line, text_line, filler_line)
        } else {
            // Single line header
            let total_content_width = effective_length - start_char.len() - end_char.len() - 2;
            let aligned_text = self.align_text(
                &transformed_text,
                total_content_width,
                &self.settings.main_header_align,
                &self.settings.main_header_filler,
            );
            format!("{}{} {} {}", indent, start_char, aligned_text, end_char)
        }
    }

    fn make_subheader(&self, text: &str, language: &str, indent_size: usize) -> String {
        let comment_chars = self.get_comment_chars(language);
        if comment_chars.is_none() {
            return format!("// Unsupported language: {}", language);
        }

        let chars = comment_chars.unwrap();
        let start_char = &chars[0];
        let end_char = if chars.len() > 1 {
            &chars[1]
        } else {
            start_char
        };

        let effective_length = self.get_effective_length(indent_size);
        let transformed_text = self.transform_text(text, &self.settings.subheader_transform);

        let indent = " ".repeat(indent_size);
        let total_content_width = effective_length - start_char.len() - end_char.len() - 2;
        let aligned_text = self.align_text(
            &transformed_text,
            total_content_width,
            &self.settings.subheader_align,
            &self.settings.subheader_filler,
        );

        format!("{}{} {} {}", indent, start_char, aligned_text, end_char)
    }

    fn insert_solid_line(&self, language: &str, indent_size: usize) -> String {
        let comment_chars = self.get_comment_chars(language);
        if comment_chars.is_none() {
            return format!("// Unsupported language: {}", language);
        }

        let chars = comment_chars.unwrap();
        let start_char = &chars[0];
        let end_char = if chars.len() > 1 {
            &chars[1]
        } else {
            start_char
        };

        let effective_length = self.get_effective_length(indent_size);
        let indent = " ".repeat(indent_size);

        let filler_count = effective_length - start_char.len() - end_char.len() - 2;
        format!(
            "{}{} {} {}",
            indent,
            start_char,
            self.settings.line_filler.repeat(filler_count),
            end_char
        )
    }

    fn get_current_line_info(&self) -> (String, usize, String) {
        // This would typically get the current line content, language, and indentation
        // For now, returning defaults - in a real implementation, this would interface with Zed's API
        ("".to_string(), 0, "javascript".to_string())
    }

    fn replace_current_line(&self, new_content: &str) {
        // This would replace the current line with new content
        // In a real implementation, this would interface with Zed's text manipulation API
        println!("Would replace line with: {}", new_content);
    }

    fn insert_at_cursor(&self, content: &str) {
        // This would insert content at the cursor position
        // In a real implementation, this would interface with Zed's text manipulation API
        println!("Would insert: {}", content);
    }
}

impl zed::Extension for CommentDividerExtension {
    fn new() -> Self {
        Self::new()
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> Result<zed::Command, String> {
        Err("This extension doesn't provide language servers".to_string())
    }

    fn run_slash_command(
        &self,
        command: zed::SlashCommand,
        args: Vec<String>,
        _worktree: Option<&zed::Worktree>,
    ) -> Result<zed::SlashCommandOutput, String> {
        match command.name.as_str() {
            "divider" => {
                let text = args.join(" ");
                let (current_line, indent_size, language) = self.get_current_line_info();

                let result = if text.is_empty() {
                    self.insert_solid_line(&language, indent_size)
                } else {
                    self.make_subheader(&text, &language, indent_size)
                };

                Ok(zed::SlashCommandOutput {
                    text: result.clone(),
                    sections: vec![zed::SlashCommandOutputSection {
                        range: (0..result.len()).into(),
                        label: "Comment Divider".to_string(),
                    }],
                })
            }
            "header" => {
                let text = args.join(" ");
                let (_, indent_size, language) = self.get_current_line_info();

                let result = self.make_main_header(&text, &language, indent_size);

                Ok(zed::SlashCommandOutput {
                    text: result.clone(),
                    sections: vec![zed::SlashCommandOutputSection {
                        range: (0..result.len()).into(),
                        label: "Comment Header".to_string(),
                    }],
                })
            }
            _ => Err(format!("Unknown command: {}", command.name)),
        }
    }

    fn complete_slash_command_argument(
        &self,
        command: zed::SlashCommand,
        _args: Vec<String>,
    ) -> Result<Vec<zed::SlashCommandArgumentCompletion>, String> {
        match command.name.as_str() {
            "divider" | "header" => Ok(vec![zed::SlashCommandArgumentCompletion {
                label: "Custom text".to_string(),
                new_text: "".to_string(),
                run_command: false,
            }]),
            _ => Ok(vec![]),
        }
    }
}

// Register the extension
zed::register_extension!(CommentDividerExtension);
