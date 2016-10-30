module XohImNooB
  class L33tWords

    def self.make(word)
      l33t_word = word.
                    gsub('to',  '2').
                    gsub('for', '4').
                    gsub('ate', '8').
                    gsub('l',   '1').
                    gsub('e',   '3').
                    gsub('s',   '5').
                    gsub('t',   '7').
                    gsub('o',   '0')
      return word == l33t_word ? [] : [l33t_word]
    end

  end
end
