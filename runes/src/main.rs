use clap::Parser;
use std::collections::HashMap;

#[derive(Parser)]
#[command(version, about = "Rhunes translator for Gentoo Linux")]
struct Args {
    /// Text to translate to Rhunes
    text: String,

    /// Reverse translation: from Rhunes to Latin
    #[arg(short, long)]
    reverse: bool,
}

struct RhunesTranslator {
    latin_to_rhunes: HashMap<char, char>,
    rhunes_to_latin: HashMap<char, char>,
    special_rules: HashMap<String, String>,
    reverse_special_rules: HashMap<String, String>,
}

impl RhunesTranslator {
    fn new() -> Self {
        // Base character mappings (lowercase)
        let latin_chars = "abcdefghijklmnopqrstuvwyz";
        let rhunes_chars = "ᚨᛒᚲᛞᛖᚠᚷᚺᛁᛃᚲᛚᛗᚾᛟᛈᚱᛊᛏᚢᚢᚹᛁᛉ";

        let latin_to_rhunes: HashMap<char, char> = latin_chars
            .chars()
            .zip(rhunes_chars.chars())
            .collect();

        let rhunes_to_latin: HashMap<char, char> = rhunes_chars
            .chars()
            .zip(latin_chars.chars())
            .collect();

        // Special rules (multi-character mappings)
        let special_rules: HashMap<String, String> = [
            ("ä".to_string(), "ᛇ".to_string()),
            ("ö".to_string(), "ᛟᛖ".to_string()),
            ("ü".to_string(), "ᚢᛖ".to_string()),
            ("å".to_string(), "ᚨᚨ".to_string()),
            ("ø".to_string(), "ᚢᚢ".to_string()),
            ("æ".to_string(), "ᛇᛇ".to_string()),
            ("th".to_string(), "ᚦ".to_string()),
            ("sk".to_string(), "ᚺᚱᚲ".to_string()),
            ("sj".to_string(), "ᚺᚱᚲ".to_string()),
            ("x".to_string(), "ᚲᛊ".to_string()),
            ("ng".to_string(), "ᛜ".to_string()),
        ]
        .iter()
        .cloned()
        .collect();

        let reverse_special_rules: HashMap<String, String> = [
            ("ᛇ".to_string(), "ae".to_string()),
            ("ᛟᛖ".to_string(), "oe".to_string()),
            ("ᚢᛖ".to_string(), "ue".to_string()),
            ("ᚨᚨ".to_string(), "aa".to_string()),
            ("ᚢᚢ".to_string(), "uu".to_string()),
            ("ᛇᛇ".to_string(), "aeae".to_string()),
            ("ᚦ".to_string(), "th".to_string()),
            ("ᚺᚱᚲ".to_string(), "sk".to_string()), // or "sj"
            ("ᚲᛊ".to_string(), "x".to_string()),
            ("ᛜ".to_string(), "ng".to_string()),
        ]
        .iter()
        .cloned()
        .collect();

        Self {
            latin_to_rhunes,
            rhunes_to_latin,
            special_rules,
            reverse_special_rules,
        }
    }

    fn to_rhunes(&self, text: &str) -> String {
        let mut result = String::new();
        let mut chars = text.chars().peekable();

        while let Some(current_char) = chars.next() {
            let current_lower = current_char.to_ascii_lowercase();
            let mut handled = false;

            // Check for multi-character special rules
            if let Some(&next_char) = chars.peek() {
                let next_lower = next_char.to_ascii_lowercase();
                let combo = format!("{}{}", current_lower, next_lower);
                if let Some(rhunes) = self.special_rules.get(&combo) {
                    result.push_str(rhunes);
                    chars.next(); // Skip the next character
                    handled = true;
                }
            }

            // Check for single-character special rules
            if !handled {
                let current_str = current_lower.to_string();
                if let Some(rhunes) = self.special_rules.get(&current_str) {
                    result.push_str(rhunes);
                    handled = true;
                }
            }

            // Default to base mapping
            if !handled {
                if let Some(&rhune) = self.latin_to_rhunes.get(&current_lower) {
                    // Preserve case for non-special characters
                    let rhune = if current_char.is_uppercase() {
                        rhune.to_ascii_uppercase()
                    } else {
                        rhune
                    };
                    result.push(rhune);
                } else {
                    // Keep non-alphabetic characters as-is
                    result.push(current_char);
                }
            }
        }

        result
    }

    fn from_rhunes(&self, text: &str) -> String {
        let mut result = String::new();
        let mut chars = text.chars().peekable();

        while let Some(current_char) = chars.next() {
            let mut handled = false;

            // Check for multi-character Rhunes sequences
            for (rhune_seq, latin) in &self.reverse_special_rules {
                if text[chars.clone().count()..].starts_with(rhune_seq) {
                    result.push_str(latin);
                    for _ in 1..rhune_seq.len() {
                        chars.next(); // Skip consumed characters
                    }
                    handled = true;
                    break;
                }
            }

            // Default to base mapping
            if !handled {
                if let Some(&latin) = self.rhunes_to_latin.get(&current_char) {
                    result.push(latin);
                } else {
                    result.push(current_char);
                }
            }
        }

        result
    }
}

fn main() {
    let args = Args::parse();
    let translator = RhunesTranslator::new();

    if args.reverse {
        println!("{}", translator.from_rhunes(&args.text));
    } else {
        println!("{}", translator.to_rhunes(&args.text));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_translation() {
        let translator = RhunesTranslator::new();

        // Test forward translation
        let input = "När skeppen var gjorda av trä, var männen gjorda av stål.";
        let expected = "ᚾᛇᚱ ᚺᚱᚲᛈᛈᛇᚾ ᚢᚨᚱ ᚷᛃᛟᚱᛞᚨ ᚨᚢ ᛏᚱᛇ, ᚢᚨᚱ ᛗᛇᚾᚾᛇᚾ ᚷᛃᛟᚱᛞᚨ ᚨᚢ ᛊᛏᚨᚨᛚ.";
        assert_eq!(translator.to_rhunes(input), expected);

        // Test reverse translation (approximate)
        let back_result = translator.from_rhunes(expected);
        println!("Original: {}", input.to_lowercase());
        println!("Back: {}", back_result);
    }
}


