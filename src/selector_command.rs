use std::collections::HashMap;
use strsim::levenshtein;

const THRESHOLD: usize = 2;   // порог расстояния

pub fn voter_command(text: &str, key_words: &HashMap<String, Vec<String>>) -> Option<String> {
    // 1. Разбить текст на слова (нижний регистр, без знаков препинания)
    let words: Vec<String> = text
        .to_lowercase()
        .split_whitespace()
        .map(|w| w.trim_matches(|c: char| !c.is_alphanumeric()).to_string())
        .filter(|w| !w.is_empty())
        .collect();

    // 2. Для каждой команды — посчитать совпадения
    let mut best_name: Option<String> = None;
    let mut best_score = 0;

    for (name, keywords) in key_words {
        let mut score = 0;

        for keyword in keywords {
            let keyword_lower = keyword.to_lowercase();

            for word in &words {
                // Точное совпадение — жирный плюс
                if word == &keyword_lower {
                    score += 10;
                    continue;
                }

                // Левенштейн
                let distance = levenshtein(word, &keyword_lower);
                if distance <= THRESHOLD {
                    score += 10 - distance;   // чем ближе — тем больше
                }
            }
        }

        if score > best_score {
            best_score = score;
            best_name = Some(name.clone());
        }
    }

    best_name
}
