module XohImNooB
  class Awesomeness
    def initialize(words)
      @words = words
    end

    def of(digest)
      digest_downcase = digest.downcase
      best_word_list = []
      @words.each do |word|
        if digest_downcase.include? word
          word_list = [word]
          i = digest_downcase.index(word) + word.size
          word_list += finish_phrase(digest_downcase[i..-1])
          if num_letters(word_list) > num_letters(best_word_list)
            best_word_list = word_list
          end
        end
      end
      return [0, nil] if best_word_list.empty?
      n = num_letters(best_word_list)
      start = digest_downcase.index(best_word_list[0])
      awesome_str = "#{digest[start..(start + n - 1)]} (#{best_word_list.join(' ')})"
      return n, awesome_str
    end

    def num_letters(word_list)
      word_list.inject(0) { |num, w| num + w.size }
    end

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

    def found(sub_str)
      found = (!(@words.bsearch { |w| w.eql? sub_str }).nil?) ||
              (@words[0].eql? sub_str)
    end

  end

end
