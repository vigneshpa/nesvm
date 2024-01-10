use nesvm::Tick;

fn main() {
    let mut board = nesvm::motherboard::MotherBoardBuilder::new()
        .mount_memory(0, (u16::MAX as u32) + 1, (u16::MAX as usize) + 1)
        // .mount_memory(0, (u16::MAX as u32) + 1, (u16::MAX as usize) + 1) // Will panic due to overlap
        .get_board();

    loop {
        board.tick();
    }
}
