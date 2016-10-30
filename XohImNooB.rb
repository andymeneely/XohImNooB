require 'digest'
require_relative 'lib/awesomeness'
require_relative 'lib/l33t_words'

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

puts "Looking for awesome hashes..."
@words.each do |word|
  digest = Digest::SHA256.new.base64digest(word)
  awe, awesome_str = @awesomeness.of digest
  if awe > 5
    puts "base64(sha256(#{word})) = #{digest} (#{awesome_str}) @ #{awe}"
  end
end
