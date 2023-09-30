require 'date'

days_of_interest = 0

Date.new(1901).upto(Date.new(2000,12,1)) do |date|
  next unless date.day == 1
  days_of_interest += 1 if date.wday == 0
end

puts days_of_interest