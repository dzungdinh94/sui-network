module 0x42::M {

    #[allow(lint(constant_naming))]
    const Another_BadName: u64 = 42; // Should trigger a warning

    #[allow(lint(collapsible_else_if))]
    fun func1(x: u64) {
        if (x > 0) {

        } else {
            if (x < 0) {
                
            }
        }
    }
}
