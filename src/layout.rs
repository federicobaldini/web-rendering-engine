// CSS box model. All sizes are in px
use crate::css;
use crate::style;
use crate::style::StyledNode;
use std::default::Default;

fn sum<I>(iter: I) -> f32
where
  I: Iterator<Item = f32>,
{
  iter.fold(0., |a: f32, b: f32| a + b)
}
