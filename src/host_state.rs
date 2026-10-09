use crate::primitives;

pub struct HostState {
    pub voice: Box<dyn primitives::Voice>,
}
