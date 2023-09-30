require 'utils'

def palindrome?(x)
  x == x.reverse
end

def base10_palindrome?(x)
  palindrome?(x.to_s)
end

def base2_palindrome?(x)
  palindrome?(x.to_s(2))
end

pals = (1..1000000).entries.select {|x| base10_palindrome?(x) and base2_palindrome?(x) }
puts pals.sum
