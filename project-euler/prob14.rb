def proc_odd(n)
  (3*n)+1
end

def proc_even(n)
  n/2
end

def sequence_len(n)
  nterms = 1
  while n>1
    n = (n % 2 == 0) ? proc_even(n) : proc_odd(n)
    nterms += 1
  end
  nterms
end

max_terms = 1
max_num = 1

3.upto(1e6) do |n|
  nterms = sequence_len(n)
  if nterms > max_terms
    max_num = n
    max_terms = nterms
  end
end

puts max_num