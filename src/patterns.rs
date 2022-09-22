use crate::rainbow::{n, rainbow};
use rgb::RGB8;

/// Flat Rainbow Pattern
pub fn pattern1<const N: usize>(step: usize) -> impl Iterator<Item = impl Iterator<Item = RGB8>> {
    rainbow()
        .step_by(step)
        .map(|c| core::iter::repeat(c).take(N))
}

/// Bouncing Rainbow Pattern
pub fn pattern2<const N: usize>(step: usize) -> impl Iterator<Item = impl Iterator<Item = RGB8>> {
    rainbow().step_by(step).flat_map(|c| {
        core::iter::empty()
            .chain((0..=N).map(move |ii| {
                core::iter::repeat(c)
                    .take(ii)
                    .chain(core::iter::repeat(RGB8::new(0, 0, 0)).take(N - ii))
            }))
            .chain((0..=N).map(move |ii| {
                core::iter::repeat(RGB8::new(0, 0, 0))
                    .take(ii)
                    .chain(core::iter::repeat(c).take(N - ii))
            }))
            .chain((0..=N).rev().map(move |ii| {
                core::iter::repeat(RGB8::new(0, 0, 0))
                    .take(ii)
                    .chain(core::iter::repeat(c).take(N - ii))
            }))
            .chain((0..=N).rev().map(move |ii| {
                core::iter::repeat(c)
                    .take(ii)
                    .chain(core::iter::repeat(RGB8::new(0, 0, 0)).take(N - ii))
            }))
    })
}

/// Shifting Rainbow Pattern
pub fn pattern3<const N: usize>(step: usize) -> impl Iterator<Item = impl Iterator<Item = RGB8>> {
    rainbow().step_by(step).map(move |c| {
        core::iter::repeat(RGB8::new(0, 0, 0))
            .intersperse(c)
            .skip((c > RGB8::new(128, 128, 128)).into())
            .take(N)
    })
}

/// Scrolling Rainbow Pattern
pub fn pattern4v1<const N: usize>(step: usize) -> impl Iterator<Item = impl Iterator<Item = RGB8>> {
    rainbow()
        .step_by(step)
        .map(move |c| rainbow().skip_while(move |&x| x != c).step_by(step).take(N))
}

/// Scrolling Rainbow Pattern
pub fn pattern4v2<const N: usize>(step: usize) -> impl Iterator<Item = impl Iterator<Item = RGB8>> {
    core::iter::repeat(()).scan(rainbow().step_by(step), |state, _| {
        // Get Result
        let result = Some(state.clone().take(N));

        // Advance State
        state.next();

        // Return
        result
    })
}

/// Scrolling Rainbow Pattern
pub fn pattern4v3<const N: usize>(step: usize) -> impl Iterator<Item = impl Iterator<Item = RGB8>> {
    rainbow().step_by(step).map(move |c| {
        core::iter::successors(Some(c), |x| Some(n(x)))
            .step_by(step)
            .take(N)
    })
}
