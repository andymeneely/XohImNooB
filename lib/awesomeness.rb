module XohImNooB
  class Awesomeness
    def initialize(words)
      @words = words
    end

    def of(digest)
      awe = 0
      num_words = 0
      num_letters = 0
      left = 1000
      right = -1000
      digest_downcase = digest.downcase
      @words.each do |word|
        if digest_downcase.include? word
          num_words += 1
          num_letters += word.size

          word_start = digest_downcase.index(word)
          left = word_start < left ? word_start : left

          word_end = word_start + word.size
          right = word_end > right ? word_end : right

          # puts "word: #{word}\nnum_words: #{num_words}\nnum_letters: #{num_letters}\nword_start: #{word_start}\nword_end: #{word_end}\nleft: #{left}\nright: #{right}\n\n"
        end
      end
      return [0, nil] if num_words == 0
      return (num_letters ** num_words) / (right - left - num_letters + 1)
    end
  end

end
