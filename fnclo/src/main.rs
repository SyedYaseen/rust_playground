fn main() {
    let b = bar::<i32>; // A function item specialized for `i32`.
    println!("sz: {}", std::mem::size_of_val(&b));

    // `b` is coerced to a function pointer because `baz` expects this exact signature.
    baz(b);

    ber(bar::<u32>);

    let fn_once = biz;

    // This works because the function item is `Copy`, so each call passes a new copy to `quox_once`.
    quox_once(fn_once);
    quox_once(fn_once);

    // A closure that consumes a captured value can be passed to `quox_once` only once.
    let t = String::from("hello");
    let cl = || drop(t); // Calling this moves `t` out of the closure, so the closure implements only `FnOnce`.
    quox_once(cl);
    // quox_once(cl); // Error: the first call moved `cl`, which is not `Copy` because it owns `t`.

    let t = String::from("hello");
    let cl1 = || println!("{}", t);
    // `cl1` only reads its capture and implements `Fn`; borrowing it lets `quox_fn` call it repeatedly.
    quox_fn(&cl1);
    quox_fn(&cl1);

    // Closure trait inference examples:
    let read = || println!("{t}"); // Reads a capture: `Fn`.
    let mut count = 0;
    let mut mutate = move || count += 1; // Mutates a capture: `FnMut`.
    mutate();
    println!("COunte: {}", count);

    let owned = String::from("hello");
    let consume = || drop(owned); // Moves a capture: `FnOnce`.
    quox_once(consume);

    let mut owned = String::from("help");
    let mut mut_closure = || owned.clear();
    quox_mut(&mut mut_closure);
    println!("mutable closure: {}", owned);

    // `move` controls capture ownership, not the callable trait by itself:
    let label = String::from("hello");
    let still_fn = move || println!("{label}"); // Owns its capture but only reads it: `Fn`.

    make_fn()();
}

fn bar<T>(inp: T) -> T {
    inp
}

/*
 * `fn(i32) -> i32` requires a function pointer with this signature.
 */
fn baz(f: fn(i32) -> i32) {
    println!("{}", f(3));
}

fn biz() {
    println!("This");
}

/*
 * `Fn(u32) -> u32` accepts any value that implements the `Fn` trait with this signature.
 */
fn ber<F>(f: F)
where
    F: Fn(u32) -> u32,
{
    println!("{}", f(4));
}

fn quox_once<F>(f: F)
where
    F: FnOnce(),
{
    f();
}

fn quox_fn<F>(f: &F)
where
    F: Fn(),
{
    f();
}

fn quox_mut<F>(f: &mut F)
where
    F: FnMut(),
{
    f();
}

fn make_fn() -> impl Fn() {
    let x = String::from("x");
    let z = String::from("z");
    let cl = move || println!("{} {}", x, z);
    println!("{}", x);
    cl
}

/* Notes
 * There are three closure traits:
 * fn pointers implement all three of these traits
 *
 * Fn:
 * takes `&self`, so it can be called repeatedly without mutating or consuming the closure.
 *
 * FnMut:
 * takes `&mut self`, so it can be called repeatedly, but requires mutable access to the closure.
 *
 * FnOnce:
 * takes `self`, consuming the closure when it is called; it can therefore be called only once.
 *
 * Rust infers which traits a closure implements from how its body uses captured values:
 * - Immutable reads or borrows allow `Fn` (and therefore also `FnMut` and `FnOnce`).
 * - Mutable borrows or mutations allow `FnMut` (and therefore also `FnOnce`).
 * - Moving a captured value out of the closure allows only `FnOnce`.
 * `move` affects how values are captured, but does not by itself make a closure `FnOnce`.
 * A `move` closure that only reads its captures can still implement `Fn`.
 *
 * The hierarchy describes the minimum access a closure needs when called:
 * - `Fn` needs shared access (`&self`).
 * - `FnMut` needs exclusive mutable access (`&mut self`).
 * - `FnOnce` needs ownership (`self`).
 *
 * Since an owned value can be mutably borrowed, and a mutable borrow can be shared, the traits
 * form `Fn: FnMut: FnOnce`: an `Fn` closure also satisfies `FnMut` and `FnOnce`, while an
 * `FnMut` closure also satisfies `FnOnce`. From the caller's perspective, `Fn` provides the
 * strongest reuse guarantee and `FnOnce` the weakest.
 *
 * `FnOnce` consumes the value passed to the call; this can be a temporary reference rather than
 * the original closure. For example, an `FnMut` closure can be passed as `&mut closure` to an
 * `FnOnce` API repeatedly, because each call consumes only a fresh mutable reference. A closure
 * that moves a non-`Copy` capture, however, must itself be consumed.
 *
 * Threading implications:
 * These traits describe how a closure is called; they do not, on their own, make it safe to
 * move or share the closure between threads. A closure must also be `Send` to move to another
 * thread, and `Sync` to be shared through references. Its captured values determine whether it
 * implements those traits.
 *
 * Async implications:
 * An `async` block is a stateful value whose captured data may live across an `.await` point.
 * When spawning it onto a multi-threaded executor, the resulting future generally must be
 * `Send`; values held across `.await` must therefore also be `Send`. The closure used to create
 * that future may additionally need `Fn`, `FnMut`, or `FnOnce`, depending on whether the API
 * calls it repeatedly, mutates it, or consumes it.
 */
