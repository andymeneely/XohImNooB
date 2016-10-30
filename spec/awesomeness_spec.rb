require 'awesomeness'

describe XohImNooB::Awesomeness do
  let(:awe) { XohImNooB::Awesomeness.new(%w(im noob oh)) }

  it 'computes properly for "oh"' do
    expect(awe.of('oh')).to eq  [2, 'oh (oh)']
  end

  it 'computes properly for "ohIm"' do
    expect(awe.of('ohIm')).to eq  [4, 'ohIm (oh im)']
  end

  it 'computes properly for "XohIm"' do
    expect(awe.of('XohIm')).to eq  [4, 'ohIm (oh im)']
  end

  it 'computes properly for "ohxIm"' do
    expect(awe.of('ohxIm')).to eq  [2, "Im (im)"]
  end

  it 'computes properly for "XohImNooBa"' do
    expect(awe.of('XohImNooB')). to eq [8, 'ohImNooB (oh im noob)']
  end

  it 'computes properly for "XohImNooBHFR0OVvjcYpJ3NgPQ1qq73WKhHvch0VQtg="' do
    expect(awe.of('XohImNooBHFR0OVvjcYpJ3NgPQ1qq73WKhHvch0VQtg=')). to eq [8, 'ohImNooB (oh im noob)']
  end

  it 'computes properly for "ohImXNooB"' do
    expect(awe.of('ohImXNooB')).to eq  [4, "NooB (noob)"]
  end

  context :finish_phrase do
    it 'finishes a two word phrase' do
      expect(awe.finish_phrase('ohim')).to eq(%w(oh im))
    end

    it 'finishes a one word phrase' do
      expect(awe.finish_phrase('ohxim')).to eq(%w(oh))
    end

    it 'finishes an empty phrase"' do
      expect(awe.finish_phrase('xohim')).to eq([])
    end

    it 'finishes a terminated phrase' do
      expect(awe.finish_phrase('ohimx')).to eq(%w(oh im))
    end

  end

  context :found do
    it 'finds first' do
      expect(awe.found('im')).to be true
    end

    it 'finds middle' do
      expect(awe.found('noob')).to be true
    end

    it 'finds end' do
      expect(awe.found('oh')).to be true
    end

    it 'does not find some things' do
      expect(awe.found('xoh')).to be false
    end
  end

end
