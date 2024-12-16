describe("Solana Token Locking dApp Tests", () => {
    let lockPdaKp: web3.Keypair;
    let vestingLockPdaKp: web3.Keypair;
    const input = "lock_seed";
    const lockAmount = new BN(1000);
    const lockTime = Math.floor(Date.now() / 1000) + 3600; // 1 hour from now
    const lockName = "Test Lock";
    const extraData = "Some extra data";
    const isNft = false; // Change to true if locking an NFT
    const projectTokenMint = new web3.PublicKey("F95fs6Az4oHhQMUxxbYH1anocpyVyFpHYWMjBCQECyUZ"); // Your actual mint address
    const wsolMint = new web3.PublicKey("So11111111111111111111111111111111111111112"); // WSOL mint address
  
    beforeEach(async () => {
      lockPdaKp = new web3.Keypair();
      vestingLockPdaKp = new web3.Keypair();
    });
  
    it("should create a basic token lock", async () => {
      const txHash = await pg.program.methods
        .lock_token(
          input,
          lockAmount,
          lockTime,
          lockName,
          extraData,
          isNft,
          projectTokenMint,
          wsolMint
        )
        .accounts({
          lockPda: lockPdaKp.publicKey,
          splMint: projectTokenMint,
          authority: pg.wallet.publicKey,
          lockPdaSplAta: lockPdaKp.publicKey, // Adjust if needed
          authoritySplAta: pg.wallet.publicKey, // Adjust if needed
          systemProgram: web3.SystemProgram.programId,
          associatedTokenProgram: anchor.utils.token.ASSOCIATED_TOKEN_PROGRAM_ID,
        })
        .signers([lockPdaKp])
        .rpc();
  
      console.log(`Use 'solana confirm -v ${txHash}' to see the logs`);
  
      // Confirm transaction
      await pg.connection.confirmTransaction(txHash);
  
      // Fetch the created lock PDA
      const lockPdaAccount = await pg.program.account.lockPda.fetch(lockPdaKp.publicKey);
  
      console.log("Lock PDA data:", lockPdaAccount);
  
      // Check if the lock amount is correct
      assert(lockPdaAccount.lockAmount.eq(lockAmount), "Lock amount does not match");
    });
  
    it("should extend the lock time", async () => {
      const newLockTime = lockTime + 3600; // Extend by 1 hour
  
      const txHash = await pg.program.methods
        .extend_lock_time(newLockTime)
        .accounts({
          lockPda: lockPdaKp.publicKey,
          authority: pg.wallet.publicKey,
          systemProgram: web3.SystemProgram.programId,
        })
        .rpc();
  
      console.log(`Use 'solana confirm -v ${txHash}' to see the logs`);
  
      // Confirm transaction
      await pg.connection.confirmTransaction(txHash);
  
      // Fetch the updated lock PDA
      const lockPdaAccount = await pg.program.account.lockPda.fetch(lockPdaKp.publicKey);
  
      // Check if the lock time has been extended
      assert(lockPdaAccount.endTime.eq(new BN(newLockTime)), "Lock time was not extended correctly");
    });
  
    it("should create a vesting lock", async () => {
      const vestingLockTime = Math.floor(Date.now() / 1000) + 7200; // 2 hours from now
      const firstRelease = 20.0; // 20% first release
      const vestingPeriod = 3600; // 1 hour vesting period
      const amountPerVesting = 10.0; // 10% per vesting period
      const userList = [pg.wallet.publicKey]; // List of users
      const userAmount = [lockAmount.toNumber()]; // Amount for each user
  
      const txHash = await pg.program.methods
        .lock_vesting(
          input,
          vestingLockTime,
          lockName,
          extraData,
          firstRelease,
          vestingPeriod,
          amountPerVesting,
          userList,
          userAmount
        )
        .accounts({
          lockPda: vestingLockPdaKp.publicKey,
          splMint: projectTokenMint,
          authority: pg.wallet.publicKey,
          lockPdaSplAta: vestingLockPdaKp.publicKey, // Adjust if needed
          authoritySplAta: pg.wallet.publicKey, // Adjust if needed
          systemProgram: web3.SystemProgram.programId,
          associatedTokenProgram: anchor.utils.token.ASSOCIATED_TOKEN_PROGRAM_ID,
        })
        .signers([vestingLockPdaKp])
        .rpc();
  
      console.log(`Use 'solana confirm -v ${txHash}' to see the logs`);
  
      // Confirm transaction
      await pg.connection.confirmTransaction(txHash);
  
      // Fetch the created vesting lock PDA
      const vestingLockPdaAccount = await pg.program.account.lockPda.fetch(vestingLockPdaKp.publicKey);
  
      console.log("Vesting Lock PDA data:", vestingLockPdaAccount);
  
      // Check if the lock amount is correct
      assert(vestingLockPdaAccount.lockAmount.eq(new BN(userAmount[0])), "Vesting lock amount does not match");
    });
  
    it("should handle error when trying to unlock before time", async () => {
      try {
        const txHash = await pg.program.methods
          .unlock_token(input)
          .accounts({
            lockPda: lockPdaKp.publicKey,
            authority: pg.wallet.publicKey,
            splMint: projectTokenMint,
            lockPdaSplAta: lockPdaKp.publicKey, // Adjust if needed
            authoritySplAta: pg.wallet.publicKey, // Adjust if needed
          })
          .rpc();
  
        // Confirm transaction
        await pg.connection.confirmTransaction(txHash);
        assert.fail("Expected error not thrown");
      } catch (error) {
        assert(error.message.includes("NotUnlockTime"), "Unexpected error message");
      }
    });
  
    it("should verify vesting unlock", async () => {
      // Simulate time passing for vesting unlock
      const unlockTime = Math.floor(Date.now() / 1000) + 3600; // 1 hour from now
  
      const txHash = await pg.program.methods
        .unlock_vesting(input)
        .accounts({
          lockPda: vestingLockPdaKp.publicKey,
          authority: pg.wallet.publicKey,
          splMint: projectTokenMint,
          lockPdaSplAta: vestingLockPdaKp.publicKey, // Adjust if needed
          authoritySplAta: pg.wallet.publicKey, // Adjust if needed
        })
        .rpc();
  
      console.log(`Use 'solana confirm -v ${txHash}' to see the logs`);
  
      // Confirm transaction
      await pg.connection.confirmTransaction(txHash);
  
      // Fetch the updated vesting lock PDA
      const vestingLockPdaAccount = await pg.program.account.lockPda.fetch(vestingLockPdaKp.publicKey);
  
      // Check if the unlock amount is correct
      const expectedUnlockAmount = (vestingLockPdaAccount.lockAmount.toNumber() * firstRelease) / 100;
      assert(vestingLockPdaAccount.lockAmount.eq(new BN(expectedUnlockAmount)), "Vesting unlock amount does not match");
    });
  });