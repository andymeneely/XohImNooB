require 'awesomeness'

describe XohImNooB::Awesomeness do
  let(:awe) { XohImNooB::Awesomeness.new(%w(oh im noob)) }

  it 'computes properly for "oh"' do
    expect(awe.of('oh')).to eq  (1 ** 2) / (1)
  end

  it 'computes properly for "ohIm"' do
    expect(awe.of('ohIm')).to eq  (4 ** 2) / (1)
  end

  it 'computes properly for "ohxIm"' do
    expect(awe.of('ohxIm')).to eq  (4 ** 2) / (2)
  end

  it 'computes properly for "XohIm"' do
    expect(awe.of('XohIm')).to eq  (4 ** 2) / (1)
  end

  it 'computes properly for "XohImNooB"' do
    expect(awe.of('XohImNooB')).to eq  (8 ** 3) / (1)
  end

  it 'computes properly for "ohImXNooB"' do
    expect(awe.of('ohImXNooB')).to eq  (8 ** 3) / (2)
  end

end
