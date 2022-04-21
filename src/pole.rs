use crate::POLE_SIZE_X;
use crate::POLE_SIZE_Y;

#[derive(Clone, Debug)]
pub struct Pole {
    pub field_x: usize,
    pub field_y: usize,
    pub bariers: Vec<[usize; 2]>,
}
impl Default for Pole {
    fn default() -> Self {
        // Simple battle field without bariers
        Self {
            field_x: POLE_SIZE_X,
            field_y: POLE_SIZE_Y,
            bariers: vec![],
        }
    }
}
