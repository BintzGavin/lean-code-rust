use winit::event::{WindowEvent, Modifiers};

fn test(e: WindowEvent) {
    if let WindowEvent::ModifiersChanged(m) = e {
        let state = m.state();
        let is_ctrl = state.control_key();
        let is_super = state.super_key();
    }
}
