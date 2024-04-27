require 'utils'

def factorial(x)
  (1..x).entries.mul
end

def sum_of_factorials(x)
  digits = []
  while x > 0
    parts = x.divmod(10)
    digits << parts[1]
    x = parts[0]
  end

  digits.inject(0) { |sum, z| sum += factorial(z) }
end

nums = (3..100000).entries.select { |x| sum_of_factorials(x) == x }
print nums
puts
puts nums.sum
