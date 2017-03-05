# blowfish
A programming language

Initial syntax idea
---

```fish
global cat :: int -> interface
local  bob :: cat

impl cat age:
  say_hello  :: string -> nothing
  add_age    :: int    -> int
end


impl bob age
  say_hello name
    print_ln "hello, " + name
  end

  add_age a
    pass bob age + 1
  end
end


```

-

Partly based on the Rust and Gluon compiler backend
