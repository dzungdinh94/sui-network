module 0x42::M {
    use sui::tx_context::TxContext;

    #[allow(lint(constant_naming))]
    const Another_BadName: u64 = 42; // Should trigger a warning

    #[allow(lint(public_mut_tx_context))]
    public fun mint(_ctx: &TxContext) {
        
    }
}


module sui::tx_context {
    struct TxContext has drop {}
}
