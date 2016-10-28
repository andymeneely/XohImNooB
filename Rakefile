require 'open-uri'

desc 'Download word list to en.txt'
task :default do
  File.open("en.txt", "w+") do |saved_file|
    open("https://github.com/atebits/Words/raw/master/Words/en.txt", {ssl_verify_mode: 0}) do |read_file|
      saved_file.write(read_file.read)
    end
  end
  puts "Word file saved."
end
