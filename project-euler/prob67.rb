data = []
File.new("triangle.txt").each_line do |line|
  data << line.chomp.split(" ").map { |x| x.to_i }
end

(data.length-1).downto(1) do |row|
  0.upto(data[row].length-2) do |col|
    data[row-1][col] += data[row][col..(col+1)].max
  end
end

puts data[0][0]
