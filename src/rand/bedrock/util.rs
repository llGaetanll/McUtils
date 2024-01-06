use crate::util::BlockPoint;

pub trait BedrockRandom {
    fn init(r: &mut Self) -> Self;
    fn nextf(&mut self, p: BlockPoint) -> f64;
}
