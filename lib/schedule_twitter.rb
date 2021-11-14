require 'twitter'
require 'yaml'

secrets = YAML.load(File.read("#{__dir__}/secrets.yml"))


client = Twitter::REST::Client.new do |config|
  config.consumer_key        = secrets["consumer_key"]
  config.consumer_secret     = secrets["consumer_secret"]
  config.access_token        = secrets["access_token"]
  config.access_token_secret = secrets["access_token_secret"]
end

binding.irb