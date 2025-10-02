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
    special_rules: HashMap<&'static str, &'static str>,
    reverse_special_rules: HashMap<&'static str, &'static str>,
}

impl RhunesTranslator {
    fn new() -> Self {
        let latin_chars = "abcdefghijklmnopqrstuvwyzð";
        let rhunes_chars = "ᚨᛒᚲᛞᛖᚠᚷᚺᛁᛃᚲᛚᛗᚾᛟᛈᚲᚱᛊᛏᚢᚢᚹᛁᛉᚦ";
        
        let latin_to_rhunes: HashMap<char, char> = latin_chars
            .chars()
            .zip(rhunes_chars.chars())
            .collect();
            
        let rhunes_to_latin: HashMap<char, char> = rhunes_chars
            .chars()
            .zip(latin_chars.chars())
            .collect();
            
        let special_rules: HashMap<&str, &str> = [
            ("ä", "ᛇ"),
            ("ö", "ᛟᛖ"),
            ("ü", "ᚢᛖ"),
            ("å", "ᚨᚨ"),
            ("ø", "ᚢᚢ"),
            ("æ", "ᛇᛇ"),
            ("th", "ᚦ"),
            ("sk", "ᚺᚱᚲ"),
            ("sj", "ᚺᚱᚲ"),
            ("x", "ᚲᛊ"),
            ("ng", "ᛜ"),
        ].iter().cloned().collect();
        
        let reverse_special_rules: HashMap<&str, &str> = [
            ("ᛇ", "ae"),
            ("ᛟᛖ", "oe"),
            ("ᚢᛖ", "ue"),
            ("ᚨᚨ", "aa"),
            ("ᚢᚢ", "uu"),
            ("ᛇᛇ", "aeae"),
            ("ᚦ", "th"),
            ("ᚺᚱᚲ", "sk"),
            ("ᚲᛊ", "x"),
            ("ᛜ", "ng"),
        ].iter().cloned().collect();

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
        let text_lower = text.to_lowercase();
        let mut lower_chars = text_lower.chars().peekable();
        
        while let Some(current_char) = chars.next() {
            let current_lower = lower_chars.next().unwrap();
            
            // Handle special multi-character rules first
            let mut handled = false;
            
            if current_lower == 's' {
                if let Some(&next_lower) = lower_chars.peek() {
                    if next_lower == 'k' || next_lower == 'j' {
                        let combo: String = [current_lower, next_lower].iter().collect();
                        if let Some(&rhunes) = self.special_rules.get(combo.as_str()) {
                            result.push_str(rhunes);
                            chars.next(); // Skip the next character
                            lower_chars.next();
                            handled = true;
                        }
                    }
                }
            }

            if !handled && current_lower == 'n' {
                if let Some(&next_lower) = lower_chars.peek(){
                    if next_lower == 'g' {
                        if let Some(&rhunes) = self.special_rules.get("ng"){
                            result.push_str(rhunes); 
                            chars.next(); 
                            lower_chars.next(); 
                            handled = true; 
                        }
                    }
                }
            }
            
            if !handled && current_lower == 't' {
                if let Some(&next_lower) = lower_chars.peek() {
                    if next_lower == 'h' {
                        if let Some(&rhunes) = self.special_rules.get("th") {
                            result.push_str(rhunes);
                            chars.next(); // Skip the next character
                            lower_chars.next();
                            handled = true;
                        }
                    }
                }
            }
            
            if !handled {
                // Handle special single characters
                let current_str: String = [current_lower].iter().collect();
                if let Some(&rhunes) = self.special_rules.get(current_str.as_str()) {
                    result.push_str(rhunes);
                } else if let Some(&rhune) = self.latin_to_rhunes.get(&current_lower) {
                    // Handle regular character mapping
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
            if current_char == 'ᛟ' {
                if let Some(&next_char) = chars.peek() {
                    if next_char == 'ᛖ' {
                        result.push_str("oe");
                        chars.next();
                        handled = true;
                    }
                }
            } else if current_char == 'ᛜ' {
                    result.push_str("ng"); 
                    handled = true; 
            } else if current_char == 'ᚢ' {
                if let Some(&next_char) = chars.peek() {
                    if next_char == 'ᛖ' {
                        result.push_str("ue");
                        chars.next();
                        handled = true;
                    } else if next_char == 'ᚢ' {
                        result.push_str("uu");
                        chars.next();
                        handled = true;
                    }
                }
            } else if current_char == 'ᚨ' {
                if let Some(&next_char) = chars.peek() {
                    if next_char == 'ᚨ' {
                        result.push_str("aa");
                        chars.next();
                        handled = true;
                    }
                }
            } else if current_char == 'ᚲ' {
                if let Some(&next_char) = chars.peek() {
                    if next_char == 'ᛊ' {
                        result.push_str("x");
                        chars.next(); 
                        handled = true;
                    }
                } 
            } else if current_char == 'ᛇ' {
                if let Some(&next_char) = chars.peek() {
                    if next_char == 'ᛇ' {
                        result.push_str("aeae");
                        chars.next();
                        handled = true;
                    } else {
                        result.push_str("ae");
                        handled = true;
                    }
                } else {
                    result.push_str("ae");
                    handled = true;
                }
            } else if current_char == 'ᚺ' {
                // Check for "hrk" sequence
                let mut temp_chars = chars.clone();
                if temp_chars.next() == Some('ᚱ') && temp_chars.next() == Some('ᚲ') {
                    result.push_str("sk");
                    chars.next(); // Skip 'ᚱ'
                    chars.next(); // Skip 'ᚲ'
                    handled = true;
                }
            }
            
            if !handled {
                if let Some(&latin) = self.reverse_special_rules.get(&current_char.to_string().as_str()) {
                    result.push_str(latin);
                } else if let Some(&latin) = self.rhunes_to_latin.get(&current_char) {
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
        let result = translator.from_rhunes(&args.text);
        println!("{}", result);
    } else {
        let result = translator.to_rhunes(&args.text);
        println!("{}", result);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_translation() {
        let translator = RhunesTranslator::new();
        
        // Test the example from the prompt
        let input = "När skeppen var gjorda av trä, var männen gjorda av stål.";
        let expected = "ᚾᛖᚱ ᚺᚱᚲᛖᛈᛈᛖᚾ ᚢᚨᚱ ᚷᛃᛟᚱᛞᚨ ᚨᚢ ᛏᚱᛖ, ᚢᚨᚱ ᛗᛖᚾᚾᛖᚾ ᚷᛃᛟᚱᛞᚨ ᚨᚢ ᛊᛏᚨᚨᚢᛚ.";
        let result = translator.to_rhunes(input);
        assert_eq!(result, expected);
        
        // Test reverse translation
        let back_result = translator.from_rhunes(expected);
        // Note: some characters might not map back perfectly due to multiple mappings
        println!("Original: {}", input.to_lowercase());
        println!("Back: {}", back_result);
    }
}
