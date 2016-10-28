module XohImNooB
  class Awesomeness
    def initialize(words)
      @words = words
    end

    def of(digest)
      awe = 0
      num_words = 0
      num_letters = 0
      long_word_bonus = 0
      left = 1000
      right = -1000
      digest_downcase = digest.downcase
      word_list = []
      @words.each do |word|
        if digest_downcase.include? word
          num_words += 1
          num_letters += word.size
          long_word_bonus += word.size ** word.size if word.size > 2

          word_start = digest_downcase.index(word)
          left = word_start < left ? word_start : left

          word_end = word_start + word.size
          right = word_end > right ? word_end : right

          word_list << word
          # puts "word: #{word}\nnum_words: #{num_words}\nnum_letters: #{num_letters}\nword_start: #{word_start}\nword_end: #{word_end}\nleft: #{left}\nright: #{right}\n\n"
        end
      end
      return [0, nil] if num_words == 0 || right - left != num_letters
      # denom = right - left - num_letters + 1
      # if denom == 0
      #   puts "Huh?? digest: #{digest}\nnum_words: #{num_words}\nnum_letters: #{num_letters}\nleft: #{left}\nright: #{right}\n#{word_list}\n\n"
      #   return [0, nil]
      # else
      awesomeness = (num_letters * num_words * long_word_bonus)
      awesome_str = "#{digest[left..right]} (#{word_list.join(' ')})"
      return awesomeness, awesome_str
      # end
    end
  end

end
