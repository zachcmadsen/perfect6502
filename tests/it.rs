use perfect6502::State;

#[test]
fn lda_imm() {
    let mut state = State::new().unwrap();

    state.write(0xFFFC, 0x00);
    state.write(0xFFFD, 0x02);

    state.write(0x0200, 0xA9);
    state.write(0x0201, 0xEF);

    for _ in 0..9 {
        state.step();
        state.step();
    }

    state.step();
    state.step();
    state.step();
    state.step();

    assert_eq!(state.a(), 0xEF);
}
