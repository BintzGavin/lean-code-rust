use winit::event::Modifiers;
use winit::keyboard::ModifiersState;

fn test(modifiers: Modifiers) -> ModifiersState {
    modifiers.state()
}
