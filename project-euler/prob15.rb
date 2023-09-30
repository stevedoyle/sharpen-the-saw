
def fact(n)
  n == 0 ? 1 :  n * fact(n-1)
end

puts fact(40) / (fact(20) * fact(20))

