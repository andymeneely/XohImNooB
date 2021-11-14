use rust_bert::pipelines::pos_tagging::POSModel;

pub fn awesomeness_of_pos(pos : &str) -> u32 {
  return match pos {
      // From: https://www.ling.upenn.edu/courses/Fall_2003/ling001/penn_treebank_pos.html
      "CC" => 1,      //CC	Coordinating conjunction
      "CD" => 0,      //CD	Cardinal number
      "DT" => 0,      //DT	Determiner
      "EX" => 0,      //EX	Existential there
      "FW" => 0,      //FW	Foreign word
      "IN" => 0,      //IN	Preposition or subordinating conjunction
      "JJ" => 5,      //JJ	Adjective
      "JJR" => 5,     //JJR	Adjective, comparative
      "JJS" => 5,     //JJS	Adjective, superlative
      "LS" => 0,      //LS	List item marker
      "MD" => 0,      //MD	Modal
      "NN" => 8,      //NN	Noun, singular or mass
      "NNS" => 8,     //NNS	Noun, plural
      "NNP" => 0,     //NNP	Proper noun, singular
      "NNPS" => 0,    //NNPS	Proper noun, plural
      "PDT" => 0,     //PDT	Predeterminer
      "POS" => 0,     //POS	Possessive ending
      "PRP" => 8,     //PRP	Personal pronoun
      "PRP$" => 8,    //PRP$	Possessive pronoun
      "RB" => 5,      //RB	Adverb
      "RBR" => 5,     //RBR	Adverb, comparative
      "RBS" => 5,     //RBS	Adverb, superlative
      "RP" => 0,      //RP	Particle
      "SYM" => 0,     //SYM	Symbol
      "TO" => 0,      //TO	to
      "UH" => 11,      //UH	Interjection
      "VB" => 10,      //VB	Verb, base form
      "VBD" => 10,     //VBD	Verb, past tense
      "VBG" => 10,     //VBG	Verb, gerund or present participle
      "VBN" => 10,     //VBN	Verb, past participle
      "VBP" => 10,     //VBP	Verb, non-3rd person singular present
      "VBZ" => 10,     //VBZ	Verb, 3rd person singular present
      "WDT" => 0,     //WDT	Wh-determiner
      "WP" => 0,      //WP	Wh-pronoun
      "WP$" => 0,     //WP$	Possessive wh-pronoun
      "WRB" => 0,     //WRB	Wh-adverb
      _ => 0
  }
}