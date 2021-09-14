pub fn pipe<O>(p: impl Pipeline<O>) -> O {
    p.compute()
}

pub trait Pipeline<O> {
    fn compute(self) -> O;
}

impl<I, O, AI, A, B> Pipeline<O> for (I, A, B)
where
    A: FnOnce(I) -> AI,
    B: FnOnce(AI) -> O,
{
    fn compute(self) -> O {
        let (input, a, b) = self;
        b(a(input))
    }
}

impl<I, O, AI, BI, A, B, C> Pipeline<O> for (I, A, B, C)
where
    A: FnOnce(I) -> AI,
    B: FnOnce(AI) -> BI,
    C: FnOnce(BI) -> O,
{
    fn compute(self) -> O {
        let (input, a, b, c) = self;
        c(b(a(input)))
    }
}

impl<I, O, AI, BI, CI, A, B, C, D> Pipeline<O> for (I, A, B, C, D)
where
    A: FnOnce(I) -> AI,
    B: FnOnce(AI) -> BI,
    C: FnOnce(BI) -> CI,
    D: FnOnce(CI) -> O,
{
    fn compute(self) -> O {
        let (input, a, b, c, d) = self;
        d(c(b(a(input))))
    }
}

impl<I, O, AI, BI, CI, DI, A, B, C, D, E> Pipeline<O> for (I, A, B, C, D, E)
where
    A: FnOnce(I) -> AI,
    B: FnOnce(AI) -> BI,
    C: FnOnce(BI) -> CI,
    D: FnOnce(CI) -> DI,
    E: FnOnce(DI) -> O,
{
    fn compute(self) -> O {
        let (input, a, b, c, d, e) = self;
        e(d(c(b(a(input)))))
    }
}

impl<I, O, AI, BI, CI, DI, EI, A, B, C, D, E, F> Pipeline<O> for (I, A, B, C, D, E, F)
where
    A: FnOnce(I) -> AI,
    B: FnOnce(AI) -> BI,
    C: FnOnce(BI) -> CI,
    D: FnOnce(CI) -> DI,
    E: FnOnce(DI) -> EI,
    F: FnOnce(EI) -> O,
{
    fn compute(self) -> O {
        let (input, a, b, c, d, e, f) = self;
        f(e(d(c(b(a(input))))))
    }
}

impl<I, O, AI, BI, CI, DI, EI, FI, A, B, C, D, E, F, G> Pipeline<O> for (I, A, B, C, D, E, F, G)
where
    A: FnOnce(I) -> AI,
    B: FnOnce(AI) -> BI,
    C: FnOnce(BI) -> CI,
    D: FnOnce(CI) -> DI,
    E: FnOnce(DI) -> EI,
    F: FnOnce(EI) -> FI,
    G: FnOnce(FI) -> O,
{
    fn compute(self) -> O {
        let (input, a, b, c, d, e, f, g) = self;
        g(f(e(d(c(b(a(input)))))))
    }
}

impl<I, O, AI, BI, CI, DI, EI, FI, GI, A, B, C, D, E, F, G, H> Pipeline<O>
    for (I, A, B, C, D, E, F, G, H)
where
    A: FnOnce(I) -> AI,
    B: FnOnce(AI) -> BI,
    C: FnOnce(BI) -> CI,
    D: FnOnce(CI) -> DI,
    E: FnOnce(DI) -> EI,
    F: FnOnce(EI) -> FI,
    G: FnOnce(FI) -> GI,
    H: FnOnce(GI) -> O,
{
    fn compute(self) -> O {
        let (input, a, b, c, d, e, f, g, h) = self;
        h(g(f(e(d(c(b(a(input))))))))
    }
}

impl<IN, OUT, AI, BI, CI, DI, EI, FI, GI, HI, A, B, C, D, E, F, G, H, I> Pipeline<OUT>
    for (IN, A, B, C, D, E, F, G, H, I)
where
    A: FnOnce(IN) -> AI,
    B: FnOnce(AI) -> BI,
    C: FnOnce(BI) -> CI,
    D: FnOnce(CI) -> DI,
    E: FnOnce(DI) -> EI,
    F: FnOnce(EI) -> FI,
    G: FnOnce(FI) -> GI,
    H: FnOnce(GI) -> HI,
    I: FnOnce(HI) -> OUT,
{
    fn compute(self) -> OUT {
        let (input, a, b, c, d, e, f, g, h, i) = self;
        i(h(g(f(e(d(c(b(a(input)))))))))
    }
}

impl<IN, OUT, AI, BI, CI, DI, EI, FI, GI, HI, II, A, B, C, D, E, F, G, H, I, J> Pipeline<OUT>
    for (IN, A, B, C, D, E, F, G, H, I, J)
where
    A: FnOnce(IN) -> AI,
    B: FnOnce(AI) -> BI,
    C: FnOnce(BI) -> CI,
    D: FnOnce(CI) -> DI,
    E: FnOnce(DI) -> EI,
    F: FnOnce(EI) -> FI,
    G: FnOnce(FI) -> GI,
    H: FnOnce(GI) -> HI,
    I: FnOnce(HI) -> II,
    J: FnOnce(II) -> OUT,
{
    fn compute(self) -> OUT {
        let (input, a, b, c, d, e, f, g, h, i, j) = self;
        j(i(h(g(f(e(d(c(b(a(input))))))))))
    }
}

impl<IN, OUT, AI, BI, CI, DI, EI, FI, GI, HI, II, JI, A, B, C, D, E, F, G, H, I, J, K> Pipeline<OUT>
    for (IN, A, B, C, D, E, F, G, H, I, J, K)
where
    A: FnOnce(IN) -> AI,
    B: FnOnce(AI) -> BI,
    C: FnOnce(BI) -> CI,
    D: FnOnce(CI) -> DI,
    E: FnOnce(DI) -> EI,
    F: FnOnce(EI) -> FI,
    G: FnOnce(FI) -> GI,
    H: FnOnce(GI) -> HI,
    I: FnOnce(HI) -> II,
    J: FnOnce(II) -> JI,
    K: FnOnce(JI) -> OUT,
{
    fn compute(self) -> OUT {
        let (input, a, b, c, d, e, f, g, h, i, j, k) = self;
        k(j(i(h(g(f(e(d(c(b(a(input)))))))))))
    }
}

#[test]
fn pipe_works() {
    fn add_one(a: i32) -> i32 {
        a + 1
    }

    let r0 = pipe((0, add_one, add_one));

    let r1 = pipe((0, add_one, add_one, add_one));

    let r2 = pipe((0, add_one, add_one, add_one, add_one));

    let r3 = pipe((0, add_one, add_one, add_one, add_one, add_one));

    let r4 = pipe((0, add_one, add_one, add_one, add_one, add_one, add_one));

    let r5 = pipe((
        0, add_one, add_one, add_one, add_one, add_one, add_one, add_one,
    ));
    let r6 = pipe((
        0, add_one, add_one, add_one, add_one, add_one, add_one, add_one, add_one,
    ));

    let r7 = pipe((
        0, add_one, add_one, add_one, add_one, add_one, add_one, add_one, add_one, add_one,
    ));

    let r8 = pipe((
        0, add_one, add_one, add_one, add_one, add_one, add_one, add_one, add_one, add_one, add_one,
    ));

    let r9 = pipe((
        0, add_one, add_one, add_one, add_one, add_one, add_one, add_one, add_one, add_one,
        add_one, add_one,
    ));

    debug_assert_eq!(r0, 2);
    debug_assert_eq!(r1, 3);
    debug_assert_eq!(r2, 4);
    debug_assert_eq!(r3, 5);
    debug_assert_eq!(r4, 6);
    debug_assert_eq!(r5, 7);
    debug_assert_eq!(r6, 8);
    debug_assert_eq!(r7, 9);
    debug_assert_eq!(r8, 10);
    debug_assert_eq!(r9, 11);

    let r9 = pipe((
        //
        1_i32,
        |a| a * 2,
        |a| a * 2,
        |a| a * 2,
        |a| a * 2,
        |a| a * 2,
        |a| a * 2,
        |a| a * 2,
    ));

    debug_assert_eq!(r9, 1024);
}
