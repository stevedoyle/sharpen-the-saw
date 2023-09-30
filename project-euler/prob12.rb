
require 'prime'
require 'utils'

def sum_of_divisors(n)
  Prime.prime_division(n).map {|x| x[1] + 1}.mul
end

n = 12000
tri = (1..12000).sum
while true
  sum = sum_of_divisors(tri)
  break if sum > 500
  n += 1
  tri += n
end
puts tri
