// Copyright 2013 The Lmath Developers. For a full listing of the authors,
// refer to the AUTHORS file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::cast::transmute;
use std::cmp::ApproxEq;
use std::num::{Zero, One};

use super::Dimensional;

use super::Vec3;

#[deriving(Eq)]
pub struct Vec2<T> { x: T, y: T }

impl<T> Vec2<T> {
    #[inline]
    pub fn new(x: T, y: T ) -> Vec2<T> {
        Vec2 { x: x, y: y }
    }
}

impl<T> Dimensional<T,[T,..2]> for Vec2<T> {
    #[inline]
    pub fn index<'a>(&'a self, i: uint) -> &'a T {
        &'a self.as_slice()[i]
    }

    #[inline]
    pub fn index_mut<'a>(&'a mut self, i: uint) -> &'a mut T {
        &'a mut self.as_mut_slice()[i]
    }

    #[inline]
    pub fn as_slice<'a>(&'a self) -> &'a [T,..2] {
        unsafe { transmute(self) }
    }

    #[inline]
    pub fn as_mut_slice<'a>(&'a mut self) -> &'a mut [T,..2] {
        unsafe { transmute(self) }
    }

    #[inline(always)]
    pub fn map(&self, f: &fn(&T) -> T) -> Vec2<T> {
        Vec2::new(f(self.index(0)),
                  f(self.index(1)))
    }

    #[inline(always)]
    pub fn map_mut(&mut self, f: &fn(&mut T)) {
        f(self.index_mut(0));
        f(self.index_mut(1));
        f(self.index_mut(2));
        f(self.index_mut(3));
    }
}

impl<T:Copy> Vec2<T> {
    #[inline]
    pub fn from_value(value: T) -> Vec2<T> {
        Vec2::new(value, value)
    }

    #[inline]
    pub fn swap(&mut self, a: uint, b: uint) {
        let tmp = *self.index(a);
        *self.index_mut(a) = *self.index(b);
        *self.index_mut(b) = tmp;
    }
}

impl<T:Copy + Num> Vec2<T> {
    #[inline]
    pub fn identity() -> Vec2<T> {
        Vec2::new(One::one::<T>(), One::one::<T>())
    }

    #[inline]
    pub fn zero() -> Vec2<T> {
        Vec2::new(Zero::zero::<T>(), Zero::zero::<T>())
    }

    #[inline]
    pub fn unit_x() -> Vec2<T> {
        Vec2::new(One::one::<T>(), Zero::zero::<T>())
    }

    #[inline]
    pub fn unit_y() -> Vec2<T> {
        Vec2::new(Zero::zero::<T>(), One::one::<T>())
    }

    #[inline]
    pub fn is_zero(&self) -> bool {
        *self.index(0) == Zero::zero() &&
        *self.index(1) == Zero::zero()
    }

    #[inline]
    pub fn add_t(&self, value: T) -> Vec2<T> {
        Vec2::new(*self.index(0) + value,
                  *self.index(1) + value)
    }

    #[inline]
    pub fn sub_t(&self, value: T) -> Vec2<T> {
        Vec2::new(*self.index(0) - value,
                  *self.index(1) - value)
    }

    #[inline]
    pub fn mul_t(&self, value: T) -> Vec2<T> {
        Vec2::new(*self.index(0) * value,
                  *self.index(1) * value)
    }

    #[inline]
    pub fn div_t(&self, value: T) -> Vec2<T> {
        Vec2::new(*self.index(0) / value,
                  *self.index(1) / value)
    }

    #[inline]
    pub fn rem_t(&self, value: T) -> Vec2<T> {
        Vec2::new(*self.index(0) % value,
                  *self.index(1) % value)
    }

    #[inline]
    pub fn add_v(&self, other: &Vec2<T>) -> Vec2<T> {
        Vec2::new(*self.index(0) + *other.index(0),
                  *self.index(1) + *other.index(1))
    }

    #[inline]
    pub fn sub_v(&self, other: &Vec2<T>) -> Vec2<T> {
        Vec2::new(*self.index(0) - *other.index(0),
                  *self.index(1) - *other.index(1))
    }

    #[inline]
    pub fn mul_v(&self, other: &Vec2<T>) -> Vec2<T> {
        Vec2::new(*self.index(0) * *other.index(0),
                  *self.index(1) * *other.index(1))
    }

    #[inline]
    pub fn div_v(&self, other: &Vec2<T>) -> Vec2<T> {
        Vec2::new(*self.index(0) / *other.index(0),
                  *self.index(1) / *other.index(1))
    }

    #[inline]
    pub fn rem_v(&self, other: &Vec2<T>) -> Vec2<T> {
        Vec2::new(*self.index(0) % *other.index(0),
                  *self.index(1) % *other.index(1))
    }

    #[inline]
    pub fn neg_self(&mut self) {
        *self.index_mut(0) = -*self.index(0);
        *self.index_mut(1) = -*self.index(1);
    }

    #[inline]
    pub fn add_self_t(&mut self, value: T) {
        *self.index_mut(0) += value;
        *self.index_mut(1) += value;
    }

    #[inline]
    pub fn sub_self_t(&mut self, value: T) {
        *self.index_mut(0) -= value;
        *self.index_mut(1) -= value;
    }

    #[inline]
    pub fn mul_self_t(&mut self, value: T) {
        *self.index_mut(0) *= value;
        *self.index_mut(1) *= value;
    }

    #[inline]
    pub fn div_self_t(&mut self, value: T) {
        *self.index_mut(0) /= value;
        *self.index_mut(1) /= value;
    }

    #[inline]
    pub fn rem_self_t(&mut self, value: T) {
        *self.index_mut(0) %= value;
        *self.index_mut(1) %= value;
    }

    #[inline]
    pub fn add_self_v(&mut self, other: &Vec2<T>) {
        *self.index_mut(0) += *other.index(0);
        *self.index_mut(1) += *other.index(1);
    }

    #[inline]
    pub fn sub_self_v(&mut self, other: &Vec2<T>) {
        *self.index_mut(0) -= *other.index(0);
        *self.index_mut(1) -= *other.index(1);
    }

    #[inline]
    pub fn mul_self_v(&mut self, other: &Vec2<T>) {
        *self.index_mut(0) *= *other.index(0);
        *self.index_mut(1) *= *other.index(1);
    }

    #[inline]
    pub fn div_self_v(&mut self, other: &Vec2<T>) {
        *self.index_mut(0) /= *other.index(0);
        *self.index_mut(1) /= *other.index(1);
    }

    #[inline]
    pub fn rem_self_v(&mut self, other: &Vec2<T>) {
        *self.index_mut(0) /= *other.index(0);
        *self.index_mut(1) /= *other.index(1);
    }

    #[inline]
    pub fn dot(&self, other: &Vec2<T>) -> T {
        *self.index(0) * *other.index(0) +
        *self.index(1) * *other.index(1)
    }

    #[inline]
    pub fn perp_dot(&self, other: &Vec2<T>) -> T {
        (*self.index(0) * *other.index(1)) -
        (*self.index(1) * *other.index(0))
    }

    #[inline]
    pub fn to_homogeneous(&self) -> Vec3<T> {
        Vec3::new(self.x, self.y, Zero::zero())
    }
}

impl<T:Copy + Num> Neg<Vec2<T>> for Vec2<T> {
    #[inline]
    pub fn neg(&self) -> Vec2<T> {
        Vec2::new(-self.index(0), -self.index(1))
    }
}

impl<T:Copy + Real> Vec2<T> {
    #[inline]
    pub fn length2(&self) -> T {
        self.dot(self)
    }

    #[inline]
    pub fn length(&self) -> T {
        self.length2().sqrt()
    }

    #[inline]
    pub fn distance2(&self, other: &Vec2<T>) -> T {
        other.sub_v(self).length2()
    }

    #[inline]
    pub fn distance(&self, other: &Vec2<T>) -> T {
        other.distance2(self).sqrt()
    }

    #[inline]
    pub fn angle(&self, other: &Vec2<T>) -> T {
        self.perp_dot(other).atan2(self.dot(other))
    }

    #[inline]
    pub fn normalize(&self) -> Vec2<T> {
        self.mul_t(One::one::<T>()/self.length())
    }

    #[inline]
    pub fn normalize_to(&self, length: T) -> Vec2<T> {
        self.mul_t(length / self.length())
    }

    #[inline]
    pub fn lerp(&self, other: &Vec2<T>, amount: T) -> Vec2<T> {
        self.add_v(&other.sub_v(self).mul_t(amount))
    }

    #[inline]
    pub fn normalize_self(&mut self) {
        let n = One::one::<T>() / self.length();
        self.mul_self_t(n);
    }

    #[inline]
    pub fn normalize_self_to(&mut self, length: T) {
        let n = length / self.length();
        self.mul_self_t(n);
    }

    pub fn lerp_self(&mut self, other: &Vec2<T>, amount: T) {
        let v = other.sub_v(self).mul_t(amount);
        self.add_self_v(&v);
    }
}

impl<T:Copy + Eq + ApproxEq<T>> ApproxEq<T> for Vec2<T> {
    #[inline]
    pub fn approx_epsilon() -> T {
        ApproxEq::approx_epsilon::<T,T>()
    }

    #[inline]
    pub fn approx_eq(&self, other: &Vec2<T>) -> bool {
        self.approx_eq_eps(other, &ApproxEq::approx_epsilon::<T,T>())
    }

    #[inline]
    pub fn approx_eq_eps(&self, other: &Vec2<T>, epsilon: &T) -> bool {
        self.index(0).approx_eq_eps(other.index(0), epsilon) &&
        self.index(1).approx_eq_eps(other.index(1), epsilon)
    }
}

impl<T:Copy + Ord> Vec2<T> {
    #[inline]
    pub fn lt_t(&self, value: T) -> Vec2<bool> {
        Vec2::new(*self.index(0) < value,
                  *self.index(1) < value)
    }

    #[inline]
    pub fn le_t(&self, value: T) -> Vec2<bool> {
        Vec2::new(*self.index(0) <= value,
                  *self.index(1) <= value)
    }

    #[inline]
    pub fn ge_t(&self, value: T) -> Vec2<bool> {
        Vec2::new(*self.index(0) >= value,
                  *self.index(1) >= value)
    }

    #[inline]
    pub fn gt_t(&self, value: T) -> Vec2<bool> {
        Vec2::new(*self.index(0) > value,
                  *self.index(1) > value)
    }

    #[inline]
    pub fn lt_v(&self, other: &Vec2<T>) -> Vec2<bool> {
        Vec2::new(*self.index(0) < *other.index(0),
                  *self.index(1) < *other.index(1))
    }

    #[inline]
    pub fn le_v(&self, other: &Vec2<T>) -> Vec2<bool> {
        Vec2::new(*self.index(0) <= *other.index(0),
                  *self.index(1) <= *other.index(1))
    }

    #[inline]
    pub fn ge_v(&self, other: &Vec2<T>) -> Vec2<bool> {
        Vec2::new(*self.index(0) >= *other.index(0),
                  *self.index(1) >= *other.index(1))
    }

    #[inline]
    pub fn gt_v(&self, other: &Vec2<T>) -> Vec2<bool> {
        Vec2::new(*self.index(0) > *other.index(0),
                  *self.index(1) > *other.index(1))
    }
}

impl<T:Copy + Eq> Vec2<T> {
    #[inline]
    pub fn eq_t(&self, value: T) -> Vec2<bool> {
        Vec2::new(*self.index(0) == value,
                  *self.index(1) == value)
    }

    #[inline]
    pub fn ne_t(&self, value: T) -> Vec2<bool> {
        Vec2::new(*self.index(0) != value,
                  *self.index(1) != value)
    }

    #[inline]
    pub fn eq_v(&self, other: &Vec2<T>) -> Vec2<bool> {
        Vec2::new(*self.index(0) == *other.index(0),
                  *self.index(1) == *other.index(1))
    }

    #[inline]
    pub fn ne_v(&self, other: &Vec2<T>) -> Vec2<bool> {
        Vec2::new(*self.index(0) != *other.index(0),
                  *self.index(1) != *other.index(1))
    }
}

impl Vec2<bool> {
    #[inline]
    pub fn any(&self) -> bool {
        *self.index(0) || *self.index(1)
    }

    #[inline]
    pub fn all(&self) -> bool {
        *self.index(0) && *self.index(1)
    }

    #[inline]
    pub fn not(&self) -> Vec2<bool> {
        Vec2::new(!*self.index(0), !*self.index(1))
    }
}

#[cfg(test)]
mod tests {
    use vec::*;

    #[test]
    fn test_vec2() {
        let a = Vec2 { x: 1.0, y: 2.0 };
        let b = Vec2 { x: 3.0, y: 4.0 };
        let f1 = 1.5;
        let f2 = 0.5;

        let mut mut_a = a;

        assert_eq!(Vec2::new::<float>(1.0, 2.0), a);
        assert_eq!(Vec2::from_value(1.0), Vec2::new::<float>(1.0, 1.0));

        assert_eq!(Vec2::zero(), Vec2::new::<float>(0.0, 0.0));
        assert_eq!(Vec2::unit_x(), Vec2::new::<float>(1.0, 0.0));
        assert_eq!(Vec2::unit_y(), Vec2::new::<float>(0.0, 1.0));
        assert_eq!(Vec2::identity(), Vec2::new::<float>(1.0, 1.0));

        *mut_a.index_mut(0) = 42.0;
        *mut_a.index_mut(1) = 43.0;
        assert_eq!(mut_a, Vec2::new::<float>(42.0, 43.0));
        mut_a = a;

        mut_a.swap(0, 1);
        assert_eq!(*mut_a.index(0), *a.index(1));
        assert_eq!(*mut_a.index(1), *a.index(0));
        mut_a = a;

        assert_eq!(a.x, 1.0);
        assert_eq!(a.y, 2.0);
        assert_eq!(*a.index(0), 1.0);
        assert_eq!(*a.index(1), 2.0);

        assert_eq!(-a, Vec2::new::<float>(-1.0, -2.0));
        assert_eq!(a.neg(), Vec2::new::<float>(-1.0, -2.0));

        assert!(Vec2::new::<float>(0.0, 0.0).is_zero());
        assert!(!Vec2::new::<float>(1.0, 1.0).is_zero());

        assert_eq!(a.mul_t(f1), Vec2::new::<float>( 1.5, 3.0));
        assert_eq!(a.div_t(f2), Vec2::new::<float>( 2.0, 4.0));

        assert_eq!(a.add_v(&b), Vec2::new::<float>(    4.0,     6.0));
        assert_eq!(a.sub_v(&b), Vec2::new::<float>(   -2.0,    -2.0));
        assert_eq!(a.mul_v(&b), Vec2::new::<float>(    3.0,     8.0));
        assert_eq!(a.div_v(&b), Vec2::new::<float>(1.0/3.0, 2.0/4.0));

        mut_a.neg_self();
        assert_eq!(mut_a, -a);
        mut_a = a;

        mut_a.mul_self_t(f1);
        assert_eq!(mut_a, a.mul_t(f1));
        mut_a = a;

        mut_a.div_self_t(f2);
        assert_eq!(mut_a, a.div_t(f2));
        mut_a = a;

        mut_a.add_self_v(&b);
        assert_eq!(mut_a, a.add_v(&b));
        mut_a = a;

        mut_a.sub_self_v(&b);
        assert_eq!(mut_a, a.sub_v(&b));
        mut_a = a;

        mut_a.mul_self_v(&b);
        assert_eq!(mut_a, a.mul_v(&b));
        mut_a = a;

        mut_a.div_self_v(&b);
        assert_eq!(mut_a, a.div_v(&b));
    }

    #[test]
    fn test_vec2_approx_eq() {
        assert!(!Vec2::new::<float>(0.000001, 0.000001).approx_eq(&Vec2::new::<float>(0.0, 0.0)));
        assert!(Vec2::new::<float>(0.0000001, 0.0000001).approx_eq(&Vec2::new::<float>(0.0, 0.0)));
    }

    #[test]
    fn test_vec2_euclidean() {
        let a = Vec2::new::<float>(5.0, 12.0); // (5, 12, 13) Pythagorean triple
        let b0 = Vec2::new::<float>(3.0, 4.0); // (3, 4, 5) Pythagorean triple
        let b = a.add_v(&b0);

        assert_eq!(a.length(), 13.0);
        assert_eq!(a.length2(), 13.0 * 13.0);

        assert_eq!(b0.length(), 5.0);
        assert_eq!(b0.length2(), 5.0 * 5.0);

        assert_eq!(a.distance(&b), 5.0);
        assert_eq!(a.distance2(&b), 5.0 * 5.0);

        assert!(Vec2::new::<float>(1.0, 0.0).angle(&Vec2::new::<float>(0.0, 1.0)).approx_eq(&Real::frac_pi_2()));
        assert!(Vec2::new::<float>(10.0, 0.0).angle(&Vec2::new::<float>(0.0, 5.0)).approx_eq(&Real::frac_pi_2()));
        assert!(Vec2::new::<float>(-1.0, 0.0).angle(&Vec2::new::<float>(0.0, 1.0)).approx_eq(&-Real::frac_pi_2::<float>()));

        assert!(Vec2::new::<float>(3.0, 4.0).normalize().approx_eq(&Vec2::new::<float>(3.0/5.0, 4.0/5.0)));
        // TODO: test normalize_to, normalize_self, and normalize_self_to

        let c = Vec2::new::<float>(-2.0, -1.0);
        let d = Vec2::new::<float>( 1.0,  0.0);

        assert_eq!(c.lerp(&d, 0.75), Vec2::new::<float>(0.250, -0.250));

        let mut mut_c = c;
        mut_c.lerp_self(&d, 0.75);
        assert_eq!(mut_c, c.lerp(&d, 0.75));
    }

    #[test]
    fn test_vec2_boolean() {
        let tf = Vec2::new(true, false);
        let ff = Vec2::new(false, false);
        let tt = Vec2::new(true, true);

        assert_eq!(tf.any(), true);
        assert_eq!(tf.all(), false);
        assert_eq!(tf.not(), Vec2::new(false, true));

        assert_eq!(ff.any(), false);
        assert_eq!(ff.all(), false);
        assert_eq!(ff.not(), Vec2::new(true, true));

        assert_eq!(tt.any(), true);
        assert_eq!(tt.all(), true);
        assert_eq!(tt.not(), Vec2::new(false, false));
    }
}