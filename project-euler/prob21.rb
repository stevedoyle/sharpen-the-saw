require 'prime'

class Array
  def sum
    inject(0) {|x,y| x+y}
  end
end

def sum_of_divisors(n)
#  Prime.prime_division(n).inject(1) {|total, x| total * (x[1] + 1)}
  1.upto(n/2).select { |x| n%x == 0 }.sum
end

puts sum_of_divisors(220)
puts sum_of_divisors(284)

amicable = 1.upto(10000-1).map do |x|
  dx = sum_of_divisors(x)
  ((dx!=x) and (x == sum_of_divisors(dx))) ? x : 0
end.uniq

puts amicable.to_s
puts amicable.sum
