require 'prime'

def print_prime_factors(num)
  print "Prime factors of #{num}: "
  puts Prime.prime_division(num).flatten.reject { |x| x == 1 }.to_s  
end

def largest_prime_factor(num)
  Prime.prime_division(num).flatten.reject { |x| x == 1 }.max  
end

print_prime_factors(13195)
print_prime_factors(600851475143)
puts largest_prime_factor(600851475143)