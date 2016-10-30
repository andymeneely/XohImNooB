require 'l33t_words'

describe XohImNooB::L33tWords do
  let (:l33t) { XohImNooB::L33tWords }
  it 'makes an empty array for a non-1337 word' do
    expect(l33t.make('q')).to eq([])
  end

  it 'makes one word on one number' do
    expect(l33t.make('go')).to eq(['g0'])
  end

  it 'converts all 1337 occurrences' do
    expect(l33t.make('leetso')).to eq(['133750'])
  end

  it 'prioritizes phrases over letters' do
    expect(l33t.make('mate')).to eq(['m8'])
    expect(l33t.make('before')).to eq(['b343'])
  end

end
