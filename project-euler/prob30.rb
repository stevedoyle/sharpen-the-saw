require 'utils'

def sum_of_fifth_powers(x)
  digits = []
  while x > 0
    digits << x%10
    x /= 10
  end

  digits.inject(0) { |sum, z| sum += z**5 }
end

nums = (2..295245).entries.select { |x| sum_of_fifth_powers(x) == x }
print nums
puts
puts nums.sum
