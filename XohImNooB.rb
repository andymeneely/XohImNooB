require 'digest'

raise 'No word file - run `rake` to download word list' unless File.exists('en.txt')

@words = File.read('en.txt').split.select {|word| word.size > 4}

def has_word?(digest)
  @words.each do |word|
    return word if digest.downcase.include?(word)
  end
  return false
end

1.upto(2) do |num|
  @words.each do |word|
    digest = Digest::SHA256.new.base64digest(word)
    if (included_word = has_word?(digest)) or digest.include?('NooB')
      puts "base64(sha256(#{word})) = #{digest} (hehe #{included_word})"
    end
  end
end
