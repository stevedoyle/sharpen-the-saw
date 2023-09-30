require 'utils'

def factorial(n)
  (1..n).mul
end

def binomial(n,r)
  factorial(n)/((factorial(r) * (factorial(n-r))))
end

count = 0

23.upto(100) do |n|
  1.upto(n) do |r|
    count += 1 if binomial(n,r) > 1e6
  end
end

puts count