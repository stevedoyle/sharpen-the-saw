
fn1 = 1
fn2 = 1
n = 3

roi = ("1"+("0"*999)).to_i

while true
  fn = fn1 + fn2
  fn2 = fn1
  fn1 = fn
  n += 1
  next unless fn > roi
  puts n-1
  break
end
