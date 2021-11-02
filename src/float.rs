pub trait Float:
    rustfft::FftNum
    + num_traits::Float
    + num_traits::FloatConst
    + num_traits::Num
    + num_traits::NumAssignOps
    + num_traits::AsPrimitive<i64>
    + std::default::Default
    + std::iter::Sum
{
    fn v(x: f64) -> Self {
        Self::from_f64(x).unwrap()
    }
}

impl<
        T: rustfft::FftNum
            + num_traits::Float
            + num_traits::FloatConst
            + num_traits::Num
            + num_traits::NumAssignOps
            + num_traits::AsPrimitive<i64>
            + std::default::Default
            + std::iter::Sum
    > Float for T
{
}
