use std::num::{Zero, Real, NumCast};
use nalgebra::traits::division_ring::DivisionRing;
use nalgebra::dim3::mat3::Mat3;
use ncollide::geom::default_geom::{Plane, Ball};
use body::volumetric::Volumetric;
use dim3::aliases;

impl<N: Real + DivisionRing + NumCast + Zero + Copy>
Volumetric<N, aliases::InertiaTensor3d<N>> for aliases::Geom3d<N>
{
  #[inline(always)]
  fn volume(&self)  -> N
  { 
    match *self
    {
      Plane(p) => p.volume(),
      Ball(b)  => b.volume()
    }
  }

  #[inline(always)]
  fn inertia(&self) -> aliases::InertiaTensor3d<N>
  {
    match *self
    {
      Plane(p) => p.inertia(),
      Ball(b)  => b.inertia()
    }
  }
}

impl<N: Real + DivisionRing + NumCast + Copy>
Volumetric<N, aliases::InertiaTensor3d<N>> for aliases::Ball3d<N>
{
  #[inline(always)]
  fn volume(&self)  -> N
  { Real::pi::<N>() * self.radius() * self.radius() * self.radius() }

  #[inline(always)]
  fn inertia(&self) -> aliases::InertiaTensor3d<N>
  {
    let _0   = Zero::zero();
    let diag = NumCast::from::<N, float>(3.0 / 5.0) * self.radius() *
                                                      self.radius();

    Mat3::new(diag, _0  , _0,
              _0  , diag, _0,
              _0  , _0  , diag)
  }
}

impl<N: Zero + Copy>
Volumetric<N, aliases::InertiaTensor3d<N>> for aliases::Plane3d<N>
{
  #[inline(always)]
  fn volume(&self)  -> N
  { Zero::zero() }

  #[inline(always)]
  fn inertia(&self) -> aliases::InertiaTensor3d<N>
  { Zero::zero() }
}
