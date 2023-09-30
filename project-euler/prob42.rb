words = []
File.new("words.txt").each_line {|line| words << line.chomp.delete('"').split(",")}
words.flatten!

longest_word = words.map {|w| w.length}.max
triangle_limit = 26*longest_word

triangles = []
(1..triangle_limit).each do |n|
  tn = n*(n+1)/2
  break if tn > triangle_limit
  triangles << tn
end

def sum(word)
  word.bytes.to_a.inject(0) { |t,c| t + (c - "A".ord + 1) }
end

count = 0
words.each {|word| count += 1 if triangles.include?(sum(word))}
puts count
