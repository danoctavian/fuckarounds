
def fuark
  1
end

puts fuark
exit

class Foo
  attr_reader :x, :y
  def initialize(x, y)
    @x, @y = x, y
  end
end

puts "shit"

f = Foo.new 1, 2

puts f.methods
#puts f.permit(:x)

# override function of hash

class String
  alias orig_reverse reverse
  def reverse()
    "a lil smth: " + orig_reverse
  end
end 

puts "wtf".reverse

# playing with the hash

class MyHash
  def[](x)
    puts x
  end
end  

mh = MyHash.new

mh["HASH"]

class Hash
  alias old_i []
  def [](k)
    r = old_i(k)
    if r == nil 
      return old_i(k.to_s)
    else 
      return r
    end
  end
end

h = {"wtf" => "kiK"}

puts h["wtf"]

