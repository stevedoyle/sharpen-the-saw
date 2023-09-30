require 'set'

def sum_of_divisors(n)
  s = 1
  t = n ** 0.5
  2.upto(t) { |i| s += i + (n/i) if n % i == 0}
  s -= t if t == t.to_i
  return s
end

total = 0
abund = Set.new
1.upto(20161) do |n|
  abund.add(n) if sum_of_divisors(n) > n
  total += n if abund.select {|x| x if abund.include?(n-x) }.empty?
end

puts total
