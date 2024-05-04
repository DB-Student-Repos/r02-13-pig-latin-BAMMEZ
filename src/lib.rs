pub fn translate(input: &str) -> String {
    input
      .split_whitespace()
      .map(|w| translate_word(w))
      .collect::<Vec<String>>()
      .join(" ")
  }
  fn translate_word(input: &str) -> String {
    let chars = input.chars();
    let mut consonant_cluster_end_idx = 0;
    let mut last = None;
    for (i, c) in chars.enumerate() {
      if let Some(idx) = match (i, c, last) {
        (_, 'u', Some('q')) => Some(i + 1),
        (1, 'r', Some('x')) => Some(0),
        (1, 't', Some('y')) => Some(0),
        (i, 'y', _) if i > 0 => Some(i),
        (_, x, _) if "aeiou".contains(x) => Some(i),
        _ => None,
      } {
        consonant_cluster_end_idx = idx;
        break;
      }
      last = Some(c);
    }
    let (consonants, remaining) = input.split_at(consonant_cluster_end_idx);
    format!("{}{}ay", remaining, consonants)
  }
  