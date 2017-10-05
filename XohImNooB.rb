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

n = 1_000_000
puts "Sampling awesomeness in #{n} hashes @ #{Time.now}..."
n.times do 
  word = @words.sample(2).join(' ')
  digest = Digest::SHA256.new.base64digest(word)
  awe, sub_str, word_list = @awesomeness.of(digest)
  if awe > 9
    digest = XohImNooB::UTFHighlight.go(digest, sub_str)
    puts "base64(sha256(#{word})) = #{digest} (#{word_list.join(' ')})@ #{awe}"
  end
end

puts "Done! #{Time.now}."

# Some good examples
# base64(sha256(acceding)) = 9aRghAa3h4GLpQsWtCQonVy50l6ZbqLw2gEvADDg8uE= --> aRghAa (arghaa)@ 6
# base64(sha256(acentrics)) = DENiUh0szFvQSfCq7j0S75pBROMCoM/w4bF4Ph1/KO4= --> ROMCoM (romcom)@ 6
# base64(sha256(actinometries)) = zILNIAhl0H+ZZIJ0vahUVLoLigoRs/0D/cY3akW00js= --> LoLigo (loligo)@ 6
# base64(sha256(yet)) = z9lt2S9+ACWxRhqvwr8K6upPebJ7GT9Jxh̭M̭s̭ṰG̭O̭j̭A̭M̭s̭= (hm st go jams)@ 10
# base64(sha256(audiometrically)) = WkM9TkCcsYATsMnsuILRICibR̭ḘC̭K̭b̭i̭b̭r̭ḘA̭m̭i0c7W0eA= (reck bib ream)@ 11
# base64(sha256(throbber)) = ILXfḓi̭m̭A̭ṰṱṰh̭A̭ṱy6lV5i9xBn1dxbzjn+wmsR7XtD2io= (di matt that)
# base64(sha256(beauteously)) = R8jw2rMṊo̭B̭g̭I̭F̭ṰS̭ṋo̭ujJZbA3oNvj3NEDNJ8o2rSACY4= (nob gifts no)@ 10