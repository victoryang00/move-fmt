module DiemTransactionTimeout {
    use 0x1::DiemTimestamp;

    struct TTL has key {
        duration_microseconds: u64,
    }
    public fun initialize(dr_account: &signer) {
        move ( dr_account, TTL { duration_microseconds: 86400000000 });
    }
    spec fun initialize {
        aborts_if sender()  = 1;
        aborts_if exists (sender());
}
}