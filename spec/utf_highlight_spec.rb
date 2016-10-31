require 'utf_highlight'

describe XohImNooB::UTFHighlight do
  let (:hi) { XohImNooB::UTFHighlight }

  it 'highlights full string properly' do
    str = 'abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789+/'
    exp = 'a̭b̭c̭ḓḙf̭g̭h̭i̭j̭k̭ḽm̭ṋo̭p̭q̭r̭s̭ṱṷv̭w̭x̭y̭z̭A̭B̭C̭ḒḘF̭G̭H̭I̭J̭K̭ḼM̭ṊO̭P̭Q̭R̭S̭ṰṶV̭W̭X̭Y̭Z̭0̭1̭2̭3̭4̭5̭6̭7̭8̭9̭+̭/̭'
    expect(hi.go(str, str)).to eq exp
  end

  it 'highlights substrings properly' do
    str = 'hello world'
    sub_str = 'lo'
    expect(hi.go(str, sub_str)).to eq 'helḽo̭ world'
  end
end
