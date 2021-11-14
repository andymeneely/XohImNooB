

pub fn decorate(hash: &str, word: &str) -> String {
  let decorated_word:String = word.chars().map(|c| match c{
      'a' => '🅐',
     'b' => '🅑',
     'c' => '🅒',
     'd' => '🅓',
     'e' => '🅔',
     'f' => '🅕',
     'g' => '🅖',
     'h' => '🅗',
     'i' => '🅘',
     'j' => '🅙',
     'k' => '🅚',
     'l' => '🅛',
     'm' => '🅜',
     'n' => '🅝',
     'o' => '🅞',
     'p' => '🅟',
     'q' => '🅠',
     'r' => '🅡',
     's' => '🅢',
     't' => '🅣',
     'u' => '🅤',
     'v' => '🅥',
     'w' => '🅦',
     'x' => '🅧',
     'y' => '🅨',
     'z' => '🅩',
     'A' => '🅐',
     'B' => '🅑',
     'C' => '🅒',
     'D' => '🅓',
     'E' => '🅔',
     'F' => '🅕',
     'G' => '🅖',
     'H' => '🅗',
     'I' => '🅘',
     'J' => '🅙',
     'K' => '🅚',
     'L' => '🅛',
     'M' => '🅜',
     'N' => '🅝',
     'O' => '🅞',
     'P' => '🅟',
     'Q' => '🅠',
     'R' => '🅡',
     'S' => '🅢',
     'T' => '🅣',
     'U' => '🅤',
     'V' => '🅥',
     'W' => '🅦',
     'X' => '🅧',
     'Y' => '🅨',
     'Z' => '🅩',
      _ => c
  }).collect();
  return hash.replace(word, decorated_word.as_str());

}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn decorate_simple() {
      let exp = decorate(&String::from("myword"), "word");
      assert_eq!("🅜🅨word", exp);
  }
  #[test]
  fn contains_works() {
      assert_ne!("🅜🅨word", "myword");
  }
}