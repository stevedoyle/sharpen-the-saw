
def num_to_text(num)
  units = %w{zero one two three four five six seven eight nine}
  teens = %w{ten eleven twelve thirteen fourteen fifteen sixteen seventeen eighteen nineteen}
  tens = %w{twenty thirty forty fifty sixty seventy eighty ninety}

  text = ""
  and_required = false
  
  while num > 0
    if num >= 1000
      text += "one thousand "
      num %= 1000
      and_required = true
    elsif num >= 100
      text += units[num / 100] + " hundred "
      num %= 100
      and_required = true
    elsif num >= 20
      text += "and " if and_required
      text += tens[(num/10)-2] + " "
      num %= 10
      and_required = false
    elsif num >= 10
      text += "and " if and_required
      text += teens[(num%10)]
      num = 0
    else
      text += "and " if and_required
      text += units[num]
      num = 0
    end
  end
  
  text
end


puts num_to_text(5)
puts num_to_text(15)
puts num_to_text(25)
puts num_to_text(100)
puts num_to_text(634)
puts num_to_text(1000)
puts num_to_text(1324)
puts num_to_text(1005)

puts num_to_text(115).gsub(' ', '').length

puts (1..1000).inject(0) { |x, y| x + num_to_text(y).gsub(' ', '').length }