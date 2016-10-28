require 'digest'
require_relative 'lib/awesomeness'

raise 'No word file - run `rake` to download word list' unless File.exists? 'en.txt'

@words = File.read('en.txt').split
@words << 'im' # ...because
@words << 'noob' # ...because
@awesomeness = XohImNooB::Awesomeness.new(@words)

1.upto(2) do |num|
  @words.each do |word|
    digest = Digest::SHA256.new.base64digest(word)
    awe, awesome_str = @awesomeness.of digest
    if awe > 200
      puts "base64(sha256(#{word})) = #{digest} (#{awesome_str}) @ #{awe}"
    end
  end
end
