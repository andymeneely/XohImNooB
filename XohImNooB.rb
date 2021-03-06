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

n = 10_000_000
puts "Sampling awesomeness in #{n} hashes @ #{Time.now}..."
n.times do |i|
  word = @words.sample(2).join(' ')
  digest = Digest::SHA256.new.base64digest(word)
  awe, sub_str, word_list = @awesomeness.of(digest)
  if awe > 9
    digest = XohImNooB::UTFHighlight.go(digest, sub_str)
    puts "base64(sha256(#{word})) = #{digest} (#{word_list.join(' ')})@ #{awe} on #{i} of #{n}"
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
# base64(sha256(quinoa keramic)) = /+VZCn6TSH61pk/Kq8OPqZXqGo̭f̭ṱI̭c̭s̭Ṋḙṱw̭H̭Y̭ZUsIHc= (of tics net why)@
# ̭̭base64(sha256(menominees mammillaria)) = 69fSQFhwdh̭i̭B̭Ṷf̭F̭y̭o̭ṊṶṰS̭Ḙf̭UKAzOhKonUSe/mtS2Z2Y= (hi buffy on uts ef)@ 14 on 358236 of 10000000
# base64(sha256(unambiguous stenographically)) = CGyfdQUSiKq3JzTpoOo+rm̭o̭a̭ṷr̭m̭m̭z̭o̭a̭fVrLl+ndR4ZE= (moa ur mm zoa)@ 10 on 359934 of 10000000
# base64(sha256(metalling legislations)) = 396/ṰA̭V̭ṷS̭i̭S̭v̭i̭m̭YKRLYIiLp3v2wuF4LjzM7vHBGO3Vk= (tav us is vim)@ 10 on 343158 of 10000000
# base64(sha256(overinsistent seaning)) = vglQcbZQA̭i̭ḙM̭O̭ṶM̭A̭Ḽp̭ṷH̭R̭I̭B̭p30msWYDdRRit1G3Hj5Q= (ai emo um alp uh rib)@ 15 on 387137 of 10000000
# base64(sha256(counterprogrammings cuneatic)) = jZhF̭o̭Y̭F̭ṷṊO̭x̭Y̭o̭p̭iKT2H+Ih7pgPHjpCTxYTQ4dReBNNQ= (foy fun oxy op)@ 11 on 424502 of 10000000