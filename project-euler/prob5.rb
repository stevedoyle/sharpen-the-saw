
def evenly_divisible(num, factors)
  factors.reject {|x| num % x == 0 }.empty?
end

num = 2520
while true  
  # If a number is evenly divisible by (11..20) it will also be evenly
  # divisible by (1..10)
  break if evenly_divisible(num, (11..20))
  num += 20
end

puts num