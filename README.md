floating-pointless
------------------
[![No Maintenance Intended](http://unmaintained.tech/badge.svg)](http://unmaintained.tech/)


Software floating-point operations in Rust.

### Why do this?

Since pretty much every computer that exists nowadays has a floating-point coprocessor, you're probably wondering: isn't this library completely pointless? The answer is "yes". This library is (as the name implies) nearly 100% useless.

So why do this? Firstly, because I want to learn more about how floating-point works, and secondly, because I have a severely twisted concept of "fun".

### Using floating-pointless in your code

Why would you do that? I see no reason you could possibly want to do that.

The Rust compiler links into platform-specific assembly routines when compiling for targets without FPUs to back its floating-point types, so if you're writing code targeting a CPU without a floating-point coprocessor, that's probably a better option. Floating-pointless is wholly for educational purposes.
