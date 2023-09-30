attempts = File.new("keylog.txt").each_line.to_a.select { |x| x.chomp }

passcode = Hash.new { |h,k| h[k] = [ [], [] ] }
attempts.each do |attempt|
  digits = attempt.chars.to_a
  
  passcode[digits[0]][1] << digits[1]
  passcode[digits[0]][1] << digits[2]

  passcode[digits[1]][0] << digits[0]
  passcode[digits[1]][1] << digits[2]

  passcode[digits[2]][0] << digits[0]
  passcode[digits[2]][0] << digits[1]
end

passcode.each_key { |k| passcode[k][0].uniq!; passcode[k][1].uniq! }

sorted = passcode.to_a.sort { |a,b| a[1][0].length <=> b[1][0].length }

sorted.each {|x| p x}
passcode = sorted.map { |x| x[0] }
puts passcode.join
