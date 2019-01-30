pub enum Pos {
  N, // 체언(대명사, 명사, 수사)
  M, // 수식언(관형사, 부사)
  I, // 독립언(감탄사)
  J, // 관계언(조사)
  V, // 용언(동사, 형용사)
  F, // 외국어
  S, // 특수문자
  U, // 분석불가
}

pub fn split_sentense(s: &str) -> Vec<&str> {
  s.split(". ").collect()
}

pub fn split_chunk(s: &str) -> Vec<&str> {
  s.split_whitespace().collect()
}

pub fn tokenize() {

}