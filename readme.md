## readme.md

`cargo check > try / expect / finally`

---

🦀 just pushing my daily rust learnings here.
<br>
i wouldn’t call this a project. it’s more like a ritual.
<br>
open terminal. write code. fight compiler. give up. try again.
<br>
sometimes it works. sometimes i pretend it does.

---

i started this as a place to throw code while i figure out how the hell rust actually works.
<br>
i still don’t know. the compiler knows. it always knows.
<br>
and it won’t let me forget.

---

there’s no roadmap here. no folders like `src/components/ui/button`.
<br>
just files like `day_12_trait_bound_wtf.rs` and `lifetimes_are_fake.rs`.
<br>
maybe one of them does something. maybe they all do nothing.
<br>
that’s not the point.

---

this is a repo for:

- stuff that broke
- stuff that compiled by accident
- experiments with traits that definitely shouldn’t work
- weird `impl<T: Trait>` things i don’t remember writing
- lifetimes i slapped `'a` on until the error went away
- and the occasional moment of “oh. oh wait. i get it now” (i don’t)

---

i’ve learned more from compiler errors than from any blog post.
<br>
not because they teach — but because they hurt.
<br>
they force you to **feel** the type system.

---

rust is like:
- “here, be safe”
- “but also here’s `unsafe`”
- “and btw, this variable doesn’t live long enough”
- “and you can’t mutate that”
- “but you can... if you wrap it in an `Rc<RefCell<Mutex<Option<T>>>>`”

cool. thanks.

---

clippy watches me like a disappointed teacher.
<br>
i write something janky, and it just goes

> “consider using `.map()` instead of this cursed `match` expression”

ok clippy. i’ll do it your way. until it breaks.

---

sometimes the most productive thing i do here is delete code.
<br>
like spiritual cleansing. remove the `Box`, gain inner peace.

---

i don’t know if this is a learning repo. or a dumping ground.
maybe both.
<br>
it’s not clean. it’s not idiomatic. it’s just me vs the compiler.
<br>
and the compiler is winning.

---

dig through the files, open random `.rs` files like you're defusing a bomb.
<br>
but don’t ask what anything does — i’m still negotiating with the compiler myself.
<br>
this repo isn’t finished. it never will be.
<br>
because in rust, the moment you think you’re done,
<br>
the borrow checker reminds you: *you’re not.*

---

## things rust has taught me

- ownership is real and it hurts
- everything is a reference to a reference to a reference
- if your code compiles, you're already better than yesterday
- the borrow checker isn’t a bug. it’s a therapist
- sometimes `unwrap()` is self-care
- `match` is both a control flow and a coping mechanism
- lifetimes aren’t real, but the trauma is
- `String` and `&str` are different. always. forever. painfully.
- cloning everything feels wrong, but silence from the compiler feels right
- `Result<T, E>` is a relationship — you need to handle it with care
- `async` in rust? no. i still need to heal from lifetimes first
- `impl<T: Trait>` looks harmless until it multiplies
- sometimes you don’t fix the bug — you just write around it

---

## things clippy has said to me in my darkest moments

- “you could simplify this with `.map()`”
<br>
  i could also go outside. but here we are.

- “this `match` can be written more cleanly”
<br>
  so can my life, clippy.

- “warning: this function has too many arguments”
<br>
  warning: this function has too many emotions.

- “consider removing this unnecessary clone”
<br>
  it’s not unnecessary. it’s emotional support.

- “variable name `x` is not descriptive”
<br>
  it stands for *existential crisis*, clippy.

- “this method returns `Result`, but the error is never handled”
<br>
  neither are my feelings, but here we are.

- “expression could be simplified with a `let` chain”
<br>
  *i* could be simplified with therapy.

- “you might want to split this function into smaller parts”
<br>
  me too, clippy. me too.

- “you can remove this `mut`”
<br>
  but i’m already too deep in the mutable lifestyle.

- “this block is empty”
<br>
  so is my soul, clippy.

- “you’ve defined this struct, but it’s never used”
<br>
  story of my career.
