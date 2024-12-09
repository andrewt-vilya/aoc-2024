use std::ops::{Add, Mul};

pub fn cartesian_product<T, U, L, R>(left: L, right: R) -> impl Iterator<Item=(T, U)> + Clone
where
    L: IntoIterator<Item=T>,
    R: IntoIterator<Item=U> + Clone,
    T: Clone,
    L::IntoIter: Clone,
    R::IntoIter: Clone,
{
    left.into_iter().flat_map(move |l|
        right.clone().into_iter().map(move |r|
            (l.clone(), r)
        )
    )
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Vec2<T=usize> { pub x: T, pub y: T }

impl<T> Vec2<T> {
    pub fn new(x: T, y: T) -> Self { Self { x, y } }

    pub fn convert<U: TryFrom<T>>(self) -> Option<Vec2<U>> {
        Some(Vec2::new(
            self.x.try_into().ok()?,
            self.y.try_into().ok()?,
        ))
    }
}

impl Vec2<isize> {
    pub fn all_directions() -> impl Iterator<Item=Self> + Clone {
        cartesian_product([-1, 0, 1], [-1, 0, 1])
            .filter(|&(x, y)| x != 0 || y != 0)
            .map(Self::from)
    }
}

impl<T, U> Add<Vec2<U>> for Vec2<T> where T: Add<U> {
    type Output = Vec2<<T as Add<U>>::Output>;
    fn add(self, rhs: Vec2<U>) -> Self::Output { Vec2::new(self.x + rhs.x, self.y + rhs.y) }
}

impl<T, U> Mul<U> for Vec2<T> where T: Mul<U>, U: Clone {
    type Output = Vec2<<T as Mul<U>>::Output>;
    fn mul(self, rhs: U) -> Self::Output { Vec2::new(self.x * rhs.clone(), self.y * rhs) }
}

impl<T> From<(T, T)> for Vec2<T> {
    fn from((x, y): (T, T)) -> Self { Self { x, y } }
}
