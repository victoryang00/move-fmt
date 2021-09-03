
module M {

  use 0x1::DiemTimestamp;

  public fun theRun(blockHash:u64,addr:address) :u64{

        let salt:u64;
        let h:u64;
        let y:u64;
        let seed:u64;

        h = 0;
        y = 0;
        salt = DiemTimestamp::now_microseconds();

        if (blockHash == 1){
            y = salt * 15 / (salt % 5);
            seed = 15 / 3;
            seed = seed + (salt * 300);
            seed = seed + y;
        };

        if (blockHash == 2){
            y = salt * 25 / (salt % 6);
            seed = 25 / 3;
            seed = seed + (salt * 300);
            seed = seed + y;
        }else{
            if (exists<salt>(addr))
            seed = 1;
        };

        h = (seed % 100) +1;
        return h;
}

}