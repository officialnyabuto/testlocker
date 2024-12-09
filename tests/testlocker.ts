import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { 
  PublicKey, 
  SystemProgram, 
  SYSVAR_RENT_PROGRAM_ID,
  Keypair 
} from "@solana/web3.js";
import { 
  TOKEN_PROGRAM_ID, 
  ASSOCIATED_TOKEN_PROGRAM_ID, 
  createMint, 
  createAccount, 
  mintTo,
  getAccount
} from "@solana/spl-token";

describe("testlocker", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Testlocker as Program;
  
  // Test state
  let mintA: PublicKey;
  let mintB: PublicKey;
  let userTokenAccount: PublicKey;
  let lockPda: PublicKey;
  let lockPdaAta: PublicKey;
  let vestingLockPda: PublicKey;
  let vestingLockPdaAta: PublicKey;
  
  // Test parameters
  const LOCK_AMOUNT = new anchor.BN(100_000_000);
  const ONE_HOUR = 3600;
  const ONE_DAY = 86400;
  
  before(async () => {
    // Create test tokens
    mintA = await createMint(
      provider.connection,
      provider.wallet.payer,
      provider.wallet.publicKey,
      null,
      9
    );
    
    mintB = await createMint(
      provider.connection,
      provider.wallet.payer,
      provider.wallet.publicKey,
      null,
      9
    );
    
    // Create user token account
    userTokenAccount = await createAccount(
      provider.connection,
      provider.wallet.payer,
      mintA,
      provider.wallet.publicKey
    );
    
    // Mint initial tokens to user
    await mintTo(
      provider.connection,
      provider.wallet.payer,
      mintA,
      userTokenAccount,
      provider.wallet.payer,
      1_000_000_000
    );
  });

  describe("Basic Token Locking", () => {
    it("Can lock tokens", async () => {
      const input = "test_seed";
      const [pda] = PublicKey.findProgramAddressSync(
        [
          Buffer.from(input),
          mintA.toBuffer(),
          provider.wallet.publicKey.toBuffer()
        ],
        program.programId
      );
      lockPda = pda;

      lockPdaAta = await anchor.utils.token.associatedAddress({
        mint: mintA,
        owner: lockPda
      });

      const lockTime = Math.floor(Date.now()/1000) + ONE_HOUR;

      await program.methods
        .lockToken(
          input,
          LOCK_AMOUNT,
          new anchor.BN(lockTime),
          "Test Lock",
          "Extra Data",
          false,
          mintA,
          mintB
        )
        .accounts({
          lockPda: lockPda,
          authority: provider.wallet.publicKey,
          splMint: mintA,
          splMintMetadataPda: mintA,
          lockPdaSplAta: lockPdaAta,
          authoritySplAta: userTokenAccount,
          tokenProgram: TOKEN_PROGRAM_ID,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
          systemProgram: SystemProgram.programId,
          rent: SYSVAR_RENT_PROGRAM_ID
        })
        .rpc();

      // Verify lock
      const lockAccount = await program.account.lockPda.fetch(lockPda);
      console.assert(lockAccount.lockAmount.toString() === LOCK_AMOUNT.toString(), "Lock amount should match");
      
      // Verify tokens transferred
      const vaultBalance = await getAccount(provider.connection, lockPdaAta);
      console.assert(vaultBalance.amount.toString() === LOCK_AMOUNT.toString(), "Vault balance should match lock amount");
    });

    it("Cannot unlock before lock time", async () => {
      try {
        await program.methods
          .unlockToken("test_seed")
          .accounts({
            lockPda: lockPda,
            authority: provider.wallet.publicKey,
            splMint: mintA,
            splMintMetadataPda: mintA,
            lockPdaSplAta: lockPdaAta,
            authoritySplAta: userTokenAccount,
            tokenProgram: TOKEN_PROGRAM_ID,
            associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
            systemProgram: SystemProgram.programId,
            rent: SYSVAR_RENT_PROGRAM_ID
          })
          .rpc();
        throw new Error("Should have thrown error");
      } catch (e) {
        console.assert(e.message.includes("NotUnlockTime"), "Should not be able to unlock before lock time");
      }
    });

    it("Can extend lock time", async () => {
      const newLockTime = Math.floor(Date.now()/1000) + ONE_DAY;
      
      await program.methods
        .extendLockTime(new anchor.BN(newLockTime))
        .accounts({
          lockPda: lockPda,
          authority: provider.wallet.publicKey,
          tokenProgram: TOKEN_PROGRAM_ID,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
          systemProgram: SystemProgram.programId,
          rent: SYSVAR_RENT_PROGRAM_ID
        })
        .rpc();

      const lockAccount = await program.account.lockPda.fetch(lockPda);
      console.assert(lockAccount.endTime.toString() === newLockTime.toString(), "Lock time should be extended");
    });
  });

  describe("Vesting Lock", () => {
    it("Can create vesting lock", async () => {
      const input = "vesting_test";
      const [pda] = PublicKey.findProgramAddressSync(
        [
          Buffer.from(input),
          mintA.toBuffer(),
          provider.wallet.publicKey.toBuffer()
        ],
        program.programId
      );
      vestingLockPda = pda;

      vestingLockPdaAta = await anchor.utils.token.associatedAddress({
        mint: mintA,
        owner: vestingLockPda
      });

      const lockTime = Math.floor(Date.now()/1000) + ONE_HOUR;
      const userList = [provider.wallet.publicKey];
      const userAmount = [new anchor.BN(50_000_000)];

      await program.methods
        .lockVesting(
          input,
          new anchor.BN(lockTime),
          "Vesting Lock",
          "Vesting Extra Data",
          20.0, // first_release percentage
          new anchor.BN(7), // vesting_period in days
          10.0, // amount_per_vesting percentage
          userList,
          userAmount
        )
        .accounts({
          lockPda: vestingLockPda,
          authority: provider.wallet.publicKey,
          splMint: mintA,
          splMintMetadataPda: mintA,
          lockPdaSplAta: vestingLockPdaAta,
          authoritySplAta: userTokenAccount,
          tokenProgram: TOKEN_PROGRAM_ID,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
          systemProgram: SystemProgram.programId,
          rent: SYSVAR_RENT_PROGRAM_ID
        })
        .rpc();

      const vestingAccount = await program.account.lockPda.fetch(vestingLockPda);
      console.assert(vestingAccount.firstRelease === 20.0, "First release percentage should match");
      console.assert(vestingAccount.amountPerVesting === 10.0, "Amount per vesting percentage should match");
    });
  });

  describe("Error Cases", () => {
    it("Cannot lock zero amount", async () => {
      try {
        const input = "zero_amount_test";
        await program.methods
          .lockToken(
            input,
            new anchor.BN(0),
            new anchor.BN(Date.now()/1000 + ONE_HOUR),
            "Test Lock",
            "Extra Data",
            false,
            mintA,
            mintB
          )
          .accounts({
            lockPda: lockPda,
            authority: provider.wallet.publicKey,
            splMint: mintA,
            splMintMetadataPda: mintA,
            lockPdaSplAta: lockPdaAta,
            authoritySplAta: userTokenAccount,
            tokenProgram: TOKEN_PROGRAM_ID,
            associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
            systemProgram: SystemProgram.programId,
            rent: SYSVAR_RENT_PROGRAM_ID
          })
          .rpc();
        throw new Error("Should have thrown error");
      } catch (e) {
        console.assert(e.message.includes("AmountZero"), "Should not be able to lock zero amount");
      }
    });

    it("Cannot lock with past time", async () => {
      try {
        const input = "past_time_test";
        await program.methods
          .lockToken(
            input,
            LOCK_AMOUNT,
            new anchor.BN(Date.now()/1000 - ONE_HOUR),
            "Test Lock",
            "Extra Data",
            false,
            mintA,
            mintB
          )
          .accounts({
            lockPda: lockPda,
            authority: provider.wallet.publicKey,
            splMint: mintA,
            splMintMetadataPda: mintA,
            lockPdaSplAta: lockPdaAta,
            authoritySplAta: userTokenAccount,
            tokenProgram: TOKEN_PROGRAM_ID,
            associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
            systemProgram: SystemProgram.programId,
            rent: SYSVAR_RENT_PROGRAM_ID
          })
          .rpc();
        throw new Error("Should have thrown error");
      } catch (e) {
        console.assert(e.message.includes("BeforeNow"), "Should not be able to lock with past time");
      }
    });
  });
});