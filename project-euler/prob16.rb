require 'utils'

puts (2**1000).to_s.chars.inject(0) { |x,y| x + y.to_i }