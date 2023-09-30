class Fibonacci

  def sequence(nterms)    
    terms = []
    x1, x2 = 0, 1
    1.upto(nterms) do
      terms << x1
      x1 += x2
      x1, x2 = x2, x1 # Swapping for the next iteration
    end
    terms
  end
  
  def [](n)
    sequence(n+1)[-1]
  end
  
end

if __FILE__ == $0

  fib = Fibonacci.new
  sum = 0
  n = 0
  fn = fib[n]
  while(fn <= 4000000)
    sum += fn if fn % 2 == 0
    n += 1
    fn = fib[n]
  end
  puts sum

end