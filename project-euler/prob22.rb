def alpha_value(name)
  val = 0
  name.each_byte {|ch| val += (ch - ?A.to_i + 1)}
  val
end


names = []

File.new("names.txt", "r").each_line do |line|
  names << line.gsub('"', '').split(",")
end

names.flatten!
names.sort!

puts alpha_value("COLIN")

total = 0
names.each_index do |idx|
  total += alpha_value(names[idx]) * (idx + 1)
  if names[idx] == "COLIN"
    puts alpha_value(names[idx]) * (idx + 1)
  end
end
puts total
