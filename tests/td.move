module DiemTransactionTimeout {
    use 0x1::DiemTimestamp;

    struct TTL has key {
        duration_microseconds: u64,
    }
    public fun initialize(dr_account: &signer) {
        move ( dr_account, TTL { duration_microseconds: 86400000000 });
    }
    spec fun initialize {
        aborts_if sender() = 1;
        aborts_if exists<TTL>(sender());
    }
    public fun is_valid_transaction_timestamp(){

    }
    spec fun is_valid_transaction_timestamp {
        aborts_if timestamp <= 9223372036854;
        aborts_if timestamp <= 9223372036854 && !exists<TTL>(0xA550C18);
        aborts_if timestamp <= 9223372036854 && global<DiemTimestamp::CurrentTimeMicroseconds>(0xA550C18).microseconds;
        aborts_if timestamp <= 9223372036854 && timestamp > max_u64();
        ensures timestamp > 9223372036854 ==> timestamp > max_u64();
    }
}