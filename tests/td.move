module DiemTransactionTimeout {
    use 0x1::DiemTimestamp;

    struct TTL has key {
        // Only transactions with timestamp in between block time and block time + duration would be accepted.
        duration_microseconds: u64,
    }

    public fun initialize(dr_account: &signer) {
        // Currently set to 1day.
        move_to(dr_account, TTL { duration_microseconds: 86400000000 });
    }

    spec fun initialize {
        aborts_if sender() != 0xA550C18;
        aborts_if exists<TTL>(sender());
        ensures global<TTL>(sender()).duration_microseconds == 86400000000;
    }

    public fun set_timeout(new_duration: u64) acquires TTL {
        let timeout = borrow_global_mut<TTL>(0xA550C18);
        timeout.duration_microseconds = new_duration;
    }

    spec fun set_timeout {
        aborts_if sender() != 0xA550C18;
        aborts_if !exists<TTL>(0xA550C18);
        ensures global<TTL>(sender()).duration_microseconds == new_duration;
    }

    public fun is_valid_transaction_timestamp(timestamp: u64): bool acquires TTL {
        // Reject timestamp greater than u64::MAX / 1_000_000;
        if (timestamp > 9223372036854) {
            return false
        };

        let current_block_time = DiemTimestamp::now_microseconds();
        let timeout = borrow_global<TTL>(0xA550C18).duration_microseconds;
        let _max_txn_time = current_block_time + timeout;

        let txn_time_microseconds = timestamp * 1000000;
        current_block_time < txn_time_microseconds
    }

    spec fun is_valid_transaction_timestamp {
        aborts_if timestamp <= 9223372036854 && !exists<DiemTimestamp::CurrentTimeMicroseconds>(0xA550C18);
        aborts_if timestamp <= 9223372036854 && !exists<TTL>(0xA550C18);
        aborts_if timestamp <= 9223372036854 && global<DiemTimestamp::CurrentTimeMicroseconds>(0xA550C18).microseconds + global<TTL>(0xA550C18).duration_microseconds > max_u64();
        aborts_if timestamp <= 9223372036854 && timestamp * 1000000 > max_u64();
        ensures timestamp > 9223372036854 ==> result == false;
        ensures timestamp <= 9223372036854 ==> result == (global<DiemTimestamp::CurrentTimeMicroseconds>(0xA550C18).microseconds < timestamp * 1000000);
    }
}
