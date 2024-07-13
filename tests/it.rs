use perfect6502::{Cycle, State};

#[test]
fn lda_imm() {
    let mut state = State::new().unwrap();

    let memory = state.memory_mut();

    memory[0xFFFC] = 0x00;
    memory[0xFFFD] = 0x02;

    memory[0x0200] = 0xA9;
    memory[0x0201] = 0xEF;

    for _ in 0..9 {
        state.step();
        state.step();
    }

    state.step();
    state.step();
    
    assert!(matches!(state.rw(), Cycle::Read));

    state.step();
    state.step();

    assert_eq!(state.a(), 0xEF);
}
