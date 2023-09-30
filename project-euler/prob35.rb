require 'prime'

def rotate(digits)
  [digits[-1], digits[0...-1]].flatten
end

def circular_prime?(x)
  digits = x.to_s.chars.to_a
  digits.length.times do
    digits = rotate(digits)
    return false if not Prime.prime?(digits.join.to_i)
  end
  return true
end

primes = Prime.each(1000000).to_a

puts "Searching ..."

cprimes = primes.select { |x| circular_prime?(x) }
print cprimes
puts
puts cprimes.length
