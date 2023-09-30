require 'utils'

limit = 25
stride = 1
n = 1
diags = [1]

(2...1001).step(2) do |stride|
  4.times do
    n += stride
    diags << n
  end
end

p diags.sum