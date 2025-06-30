## readme.md

`cargo check > try / expect / finally`

---

🦀 just pushing my daily rust learnings here.
i wouldn’t call this a project. it’s more like a ritual.
open terminal. write code. fight compiler. give up. try again.
sometimes it works. sometimes i pretend it does.

---

i started this as a place to throw code while i figure out how the hell rust actually works.
i still don’t know. the compiler knows. it always knows.
and it won’t let me forget.

---

there’s no roadmap here. no folders like `src/components/ui/button`.
just files like `day_12_trait_bound_wtf.rs` and `lifetimes_are_fake.rs`.
maybe one of them does something. maybe they all do nothing.
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
not because they teach — but because they hurt.
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
i write something janky, and it just goes
> “consider using `.map()` instead of this cursed `match` expression”

ok clippy. i’ll do it your way. until it breaks.

---

sometimes the most productive thing i do here is delete code.
like spiritual cleansing. remove the `Box`, gain inner peace.

---

i don’t know if this is a learning repo. or a dumping ground.
maybe both.
it’s not clean. it’s not idiomatic. it’s just me vs the compiler.
and the compiler is winning.

---

dig through the files, open random `.rs` files like you're defusing a bomb.

but don’t ask what anything does — i’m still negotiating with the compiler myself.

this repo isn’t finished. it never will be.
because in rust, the moment you think you’re done,
the borrow checker reminds you: *you’re not.*
