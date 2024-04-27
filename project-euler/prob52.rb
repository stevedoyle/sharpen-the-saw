def digits(n)
  n.to_s.chars.to_a
end

n = 1
while true
  dn = digits(n).sort
  if dn == digits(2*n).sort and
    dn == digits(3*n).sort and
    dn == digits(4*n).sort and
    dn == digits(5*n).sort and
    dn == digits(6*n).sort and
    break
  end
  n += 1
end
puts n
