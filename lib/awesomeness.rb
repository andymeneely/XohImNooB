require_relative 'xkcd_words'

module XohImNooB
  class Awesomeness
    def initialize(words)
      @words = words
      @xkcd_words = XKCD.words.sort
    end

    def of(digest)
      digest_downcase = digest.downcase
      best_word_list = []
      @words.each do |word|                # 1. Loop over all words in dict
        if digest_downcase.include? word   # 2. If digest has the word
          word_list = [word]
          i = digest_downcase.index(word) + word.size
          rest = digest_downcase[i..-1]    #    ...keep looking for more words
          word_list += finish_phrase(rest) #    ...in an expensive recursive call
          best_word_list = word_list if score(word_list) > score(best_word_list) 
        end
      end
      return [0, nil] if best_word_list.empty?
      n = num_letters(best_word_list)
      start = digest_downcase.index(best_word_list[0])
      sub_str = digest[start..(start + n - 1)]
      return n, sub_str, best_word_list
    end

    # My scoring system for awesomeness of a given word lists
    def score(word_list)
      word_list.inject(0) do |num, w|
        num + 
        w.size + # Overall letters in the list is good
        w.size + # Longer word that get counted more with this second one
        (is_xkcd?(w) ? w.size*w.size : 0) # Long XKCD words (top 1k) are best
      end
    end

    # Count the number of letters in an array of words
    def num_letters(word_list)
      word_list.inject(0) { |num, w| num + w.size }
    end

    # Recursively determine if we can find more and more words in the phrase
    # This is pretty greedy and slow, but whatever.
    def finish_phrase(digest_tail, word_list = [])
      return word_list if digest_tail.empty?
      digest_tail.size.downto(1).each do |i|
        sub_str = digest_tail[0..i-1]
        if found(sub_str)
          word_list << sub_str
          return finish_phrase(digest_tail[i..-1], word_list)
        end
      end
      return word_list
    end

    # Do a binary search on the giant word dictionary - should be fast
    def found(sub_str)
      @words.bsearch { |w| w >= sub_str } == sub_str
    end

    # Do a binary search on the XKCD top 1000 dictionary - should be fast
    def is_xkcd?(word)
      @xkcd_words.bsearch { |w| w >= word } == word
    end

  end

end
