require 'digest'
require_relative 'lib/awesomeness'
require_relative 'lib/l33t_words'
require_relative 'lib/utf_highlight'

raise 'No word file (run `rake` to download)' unless File.exists? 'en.txt'

puts "Prepping dictionary..."
@words = File.read('en.txt').split
@words << 'im' # ...because
@words << 'noob' # ...because
# puts "  ...adding l33t words"
# l337_words = @words.inject([]) { |arr, w| arr + XohImNooB::L33tWords.make(w) }
# @words += l337_words
puts "  ...sorting"
@words.sort!
@awesomeness = XohImNooB::Awesomeness.new(@words)
puts "Done! #{@words.size} words in dictionary"

puts "Looking for awesome hashes @ #{Time.now}..."
@words.each do |word|
# @words.sample(10000).each do |word| # choose some at random
  digest = Digest::SHA256.new.base64digest(word)
  awe, sub_str, word_list = @awesomeness.of(digest)
  if awe > 9
    # digest = XohImNooB::UTFHighlight.go(digest, sub_str)
    puts "base64(sha256(#{word})) = #{digest} (#{word_list.join(' ')})@ #{awe}"
  end
end

puts "Done! #{Time.now}"

# Some good examples
# base64(sha256(acceding)) = 9aRghAa3h4GLpQsWtCQonVy50l6ZbqLw2gEvADDg8uE= --> aRghAa (arghaa)@ 6
# base64(sha256(acentrics)) = DENiUh0szFvQSfCq7j0S75pBROMCoM/w4bF4Ph1/KO4= --> ROMCoM (romcom)@ 6
# base64(sha256(actinometries)) = zILNIAhl0H+ZZIJ0vahUVLoLigoRs/0D/cY3akW00js= --> LoLigo (loligo)@ 6
