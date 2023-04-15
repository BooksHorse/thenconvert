use std::convert::Infallible;

pub fn th_to_en(s: &str) -> Result<String, Infallible> {
  let characters = s.chars();
  let res = characters
    .map(|c| {
      match c {
        ' ' => ' ',
        'ๅ' => '1',
        '/' => '2',
        '-' => '3',
        'ภ' => '4',
        'ถ' => '5',
        'ุ' => '6',
        'ึ' => '7',
        'ค' => '8',
        'ต' => '9',
        'จ' => '0',
        'ข' => '-',
        'ช' => '=', // top row non shift
        '+' => '!',
        '๑' => '@',
        '๒' => '#',
        '๓' => '$',
        '๔' => '%',
        'ู' => '^',
        '฿' => '&',
        '๕' => '*',
        '๖' => '(',
        '๗' => ')',
        '๘' => '_',
        '๙' => '+', // top row shift
        'ๆ' => 'q',
        'ไ' => 'w',
        'ำ' => 'e',
        'พ' => 'r',
        'ะ' => 't',
        'ั' => 'y',
        'ี' => 'u',
        'ร' => 'i',
        'น' => 'o',
        'ย' => 'p',
        'บ' => '[',
        'ล' => ']',  // 2nd row non shift
        'ฃ' => '\\', //below backspace
        '๐' => 'Q',
        '"' => 'W',
        'ฎ' => 'E',
        'ฑ' => 'R',
        'ธ' => 'T',
        'ํ' => 'Y',
        '๊' => 'U',
        'ณ' => 'I',
        'ฯ' => 'O',
        'ญ' => 'P',
        'ฐ' => '{',
        ',' => '}',   // 2nd row shift
        'ฅ' => '|', //below backspace
        'ฟ' => 'a',
        'ห' => 's',
        'ก' => 'd',
        'ด' => 'f',
        'เ' => 'g',
        '้' => 'h',
        '่' => 'j',
        'า' => 'k',
        'ส' => 'l',
        'ว' => ';',
        'ง' => '\u{0027}', // 3rd row non shift
        'ฤ' => 'A',
        'ฆ' => 'S',
        'ฏ' => 'D',
        'โ' => 'F',
        'ฌ' => 'G',
        '็' => 'H',
        '๋' => 'J',
        'ษ' => 'K',
        'ศ' => 'L',
        'ซ' => ':',
        '.' => '"', // 3rd row
        'ผ' => 'z',
        'ป' => 'x',
        'แ' => 'c',
        'อ' => 'v',
        'ิ' => 'b',
        'ื' => 'n',
        'ท' => 'm',
        'ม' => ',',
        'ใ' => '.',
        'ฝ' => '/', // 4th row non shift
        '(' => 'Z',
        ')' => 'X',
        'ฉ' => 'C',
        'ฮ' => 'V',
        'ฺ' => 'B',
        '์' => 'N',
        '?' => 'M',
        'ฒ' => '<',
        'ฬ' => '>',
        'ฦ' => '?', // 4th row shift
        _ => c,
      }
    })
    .collect::<String>();

  Ok(res)
}
pub fn en_to_th(s: &str) -> Result<String,Infallible> {
  let characters = s.chars();
  let res = characters
    .map(|c| {
      match c {
        ' ' => ' ',
        '1' => 'ๅ',
        '2' => '/',
        '3' => '-',
        '4' => 'ภ',
        '5' => 'ถ',
        '6' => 'ุ',
        '7' => 'ึ',
        '8' => 'ค',
        '9' => 'ต',
        '0' => 'จ',
        '-' => 'ข',
        '=' => 'ช', // top row non shift
        '!' => '+',
        '@' => '๑',
        '#' => '๒',
        '$' => '๓',
        '%' => '๔',
        '^' => 'ู',
        '&' => '฿',
        '*' => '๕',
        '(' => '๖',
        ')' => '๗',
        '_' => '๘',
        '+' => '๙', // top row shift
        'q' => 'ๆ',
        'w' => 'ไ',
        'e' => 'ำ',
        'r' => 'พ',
        't' => 'ะ',
        'y' => 'ั',
        'u' => 'ี',
        'i' => 'ร',
        'o' => 'น',
        'p' => 'ย',
        '[' => 'บ',
        ']' => 'ล',  // 2nd row non shift
        '\\' => 'ฃ', //below backspace
        'Q' => '๐',
        'W' => '"',
        'E' => 'ฎ',
        'R' => 'ฑ',
        'T' => 'ธ',
        'Y' => 'ํ',
        'U' => '๊',
        'I' => 'ณ',
        'O' => 'ฯ',
        'P' => 'ญ',
        '{' => 'ฐ',
        '}' => ',',   // 2nd row shift
        '|' => 'ฅ', //below backspace
        'a' => 'ฟ',
        's' => 'ห',
        'd' => 'ก',
        'f' => 'ด',
        'g' => 'เ',
        'h' => '้',
        'j' => '่',
        'k' => 'า',
        'l' => 'ส',
        ';' => 'ว',
        '\u{0027}' => 'ง', // 3rd row non shift
        'A' => 'ฤ',
        'S' => 'ฆ',
        'D' => 'ฏ',
        'F' => 'โ',
        'G' => 'ฌ',
        'H' => '็',
        'J' => '๋',
        'K' => 'ษ',
        'L' => 'ศ',
        ':' => 'ซ',
        '"' => '.', // 3rd row
        'z' => 'ผ',
        'x' => 'ป',
        'c' => 'แ',
        'v' => 'อ',
        'b' => 'ิ',
        'n' => 'ื',
        'm' => 'ท',
        ',' => 'ม',
        '.' => 'ใ',
        '/' => 'ฝ', // 4th row non shift
        'Z' => '(',
        'X' => ')',
        'C' => 'ฉ',
        'V' => 'ฮ',
        'B' => 'ฺ',
        'N' => '์',
        'M' => '?',
        '<' => 'ฒ',
        '>' => 'ฬ',
        '?' => 'ฦ', // 4th row shift
        _ => c,
      }
    })
    .collect::<String>();

  Ok(res)
}
#[cfg(test)]
#[test]
fn then() {
  assert_eq!(th_to_en("ๅ/-ภถุึคตจขช"), Ok(String::from("1234567890-="))); //1
  assert_eq!(th_to_en("+๑๒๓๔ู฿๕๖๗๘๙"), Ok(String::from("!@#$%^&*()_+"))); // ^1
  assert_eq!(th_to_en("ๆไำพะัีรนยบล"), Ok(String::from("qwertyuiop[]"))); // 2
  assert_eq!(
    th_to_en(r#"๐"ฎฑธํ๊ณฯญฐ,ฅ"#),
    Ok(String::from(r#"QWERTYUIOP{}|"#))
  ); // ^2
  assert_eq!(th_to_en(r#"ฟหกดเ้่าสวง"#), Ok(String::from(r#"asdfghjkl;'"#))); // 3
  assert_eq!(th_to_en(r#"ฤฆฏโฌ็๋ษศซ."#), Ok(String::from(r#"ASDFGHJKL:""#))); // ^3
  assert_eq!(th_to_en(r#"ผปแอิืทมใฝ"#), Ok(String::from(r#"zxcvbnm,./"#))); // 4
  assert_eq!(th_to_en(r#"()ฉฮฺ์?ฒฬฦ"#), Ok(String::from(r#"ZXCVBNM<>?"#))); // ^4
}
#[test]
fn enth() {
  assert_eq!(en_to_th("1234567890-="), Ok(String::from("ๅ/-ภถุึคตจขช"))); //1
  assert_eq!(en_to_th("!@#$%^&*()_+"), Ok(String::from("+๑๒๓๔ู฿๕๖๗๘๙"))); // ^1
  assert_eq!(en_to_th("qwertyuiop[]"), Ok(String::from("ๆไำพะัีรนยบล"))); // 2
  assert_eq!(
    en_to_th(r#"QWERTYUIOP{}|"#),
    Ok(String::from(r#"๐"ฎฑธํ๊ณฯญฐ,ฅ"#))
  ); // ^2
  assert_eq!(en_to_th(r#"asdfghjkl;'"#), Ok(String::from(r#"ฟหกดเ้่าสวง"#))); // 3
  assert_eq!(en_to_th(r#"ASDFGHJKL:""#), Ok(String::from(r#"ฤฆฏโฌ็๋ษศซ."#))); // ^3
  assert_eq!(en_to_th(r#"zxcvbnm,./"#), Ok(String::from(r#"ผปแอิืทมใฝ"#))); // 4
  assert_eq!(en_to_th(r#"ZXCVBNM<>?"#), Ok(String::from(r#"()ฉฮฺ์?ฒฬฦ"#))); // ^4
}


pub trait Thenconvert {
  fn en_to_th(&self) -> String;
  fn th_to_en(&self) -> String;
}
impl Thenconvert for str {
  fn en_to_th(&self) -> String {
      en_to_th(&self).unwrap()
  }
  fn th_to_en(&self) -> String {
    th_to_en(&self).unwrap()
}
}