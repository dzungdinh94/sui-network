module 0x42::M {

    #[allow(lint(constant_naming))]
    const Another_BadName: u64 = 42; // Should trigger a warning

    #[allow(lint(consecutive_ifs))]
    fun incorrect_ifs_same_cond(x: u64) {
      // Consecutive ifs with the same condition (should trigger lint)
        if (x > 10) {
            // Some logic here
        };
        if (x > 10) {
            // Some other logic here
        };

        // Consecutive ifs with the same condition in different scope (should trigger lint)
        if (x == 3) {
            // Some logic here
        };
        // Some unrelated code
        if (x == 3) {
            // Some other logic here
        };
    }
}
