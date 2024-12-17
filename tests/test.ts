import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { web3, BN } from "@coral-xyz/anchor";

describe("Solana Token Locking dApp Tests", () => {
    // Anchor provider and program
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);
    const program = anchor.workspace.gempad_solana_lock;

    // Keypairs for PDAs
    let lockPdaKp: web3.Keypair;
    let vestingLockPdaKp: web3.Keypair;

    // Test parameters
    const input = "lock_seed";
    const lockAmount = new BN(1000 * Math.pow(10, 6)); // Assuming 6 decimal places
    const lockTime = Math.floor(Date.now() / 1000) + 3600; // 1 hour from now
    const lockName = "Test Lock";
    const extraData = "Some extra data";
    const isNft = false;
    
    // Token mints
    const projectTokenMint = new web3.PublicKey("F95fs6Az4oHhQMUxxbYH1anocpyVyFpHYWMjBCQECyUZ");
    const wsolMint = new web3.PublicKey("So11111111111111111111111111111111111111112");
    const metadataProgram = new web3.PublicKey("metaqbxxUerddd12kygU6CD9WFawWmdAKRW8QyWJYzVw");

    // Vesting parameters
    const firstRelease = 20.0; // 20% first release
    const vestingPeriod = 3600; // 1 hour vesting period
    const amountPerVesting = 10.0; // 10% per vesting period

    beforeEach(async () => {
        // Generate new keypairs for each test
        lockPdaKp = web3.Keypair.generate();
        vestingLockPdaKp = web3.Keypair.generate();
    });

    // Helper function to derive metadata PDA
    const deriveMetadataPDA = async (mint: web3.PublicKey) => {
        const [metadataPda] = web3.PublicKey.findProgramAddressSync(
            [
                Buffer.from("metadata"),
                metadataProgram.toBuffer(),
                mint.toBuffer()
            ],
            metadataProgram
        );
        return metadataPda;
    };

    it("should create a basic token lock", async () => {
        // Derive metadata PDA
        const metadataPda = await deriveMetadataPDA(projectTokenMint);

        // Derive the Associated Token Account for the lock PDA
        const [lockPdaSplAta] = web3.PublicKey.findProgramAddressSync(
            [
                provider.wallet.publicKey.toBuffer(),
                anchor.utils.token.ASSOCIATED_TOKEN_PROGRAM_ID.toBuffer(),
                projectTokenMint.toBuffer()
            ],
            anchor.utils.token.ASSOCIATED_TOKEN_PROGRAM_ID
        );

        // Derive the Authority's ATA
        const authoritySplAta = await anchor.utils.token.associatedAddress({
            mint: projectTokenMint,
            owner: provider.wallet.publicKey
        });

        try {
            const txHash = await program.methods
                .lockToken(
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
                    splMintMetadataPda: metadataPda,
                    authority: provider.wallet.publicKey,
                    owner: provider.wallet.publicKey,
                    lockPdaSplAta: lockPdaSplAta,
                    authoritySplAta: authoritySplAta,
                    tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
                    systemProgram: web3.SystemProgram.programId,
                    associatedTokenProgram: anchor.utils.token.ASSOCIATED_TOKEN_PROGRAM_ID,
                    rent: web3.SYSVAR_RENT_PUBKEY,
                })
                .signers([lockPdaKp])
                .rpc();

            console.log(`Token Lock Transaction: ${txHash}`);

            // Confirm transaction
            await provider.connection.confirmTransaction(txHash);

            // Fetch the created lock PDA
            const lockPdaAccount = await program.account.lockPda.fetch(lockPdaKp.publicKey);

            console.log("Lock PDA data:", lockPdaAccount);

            // Check if the lock amount is correct
            if (!lockPdaAccount.lockAmount.eq(lockAmount)) {
                throw new Error("Lock amount does not match");
            }
        } catch (error) {
            console.error("Error in token lock test:", error);
            throw error;
        }
    });

    it("should create a vesting lock", async () => {
        // Derive metadata PDA
        const metadataPda = await deriveMetadataPDA(projectTokenMint);

        // Derive the Associated Token Account for the vesting lock PDA
        const [vestingLockPdaSplAta] = web3.PublicKey.findProgramAddressSync(
            [
                provider.wallet.publicKey.toBuffer(),
                anchor.utils.token.ASSOCIATED_TOKEN_PROGRAM_ID.toBuffer(),
                projectTokenMint.toBuffer()
            ],
            anchor.utils.token.ASSOCIATED_TOKEN_PROGRAM_ID
        );

        // Derive the Authority's ATA
        const authoritySplAta = await anchor.utils.token.associatedAddress({
            mint: projectTokenMint,
            owner: provider.wallet.publicKey
        });

        const vestingLockTime = Math.floor(Date.now() / 1000) + 7200; // 2 hours from now
        const userList = [provider.wallet.publicKey]; // List of users
        const userAmount = [lockAmount.toNumber()]; // Amount for each user

        try {
            const txHash = await program.methods
                .lockVesting(
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
                    splMintMetadataPda: metadataPda,
                    authority: provider.wallet.publicKey,
                    lockPdaSplAta: vestingLockPdaSplAta,
                    authoritySplAta: authoritySplAta,
                    systemProgram: web3.SystemProgram.programId,
                    associatedTokenProgram: anchor.utils.token.ASSOCIATED_TOKEN_PROGRAM_ID,
                    tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
                    rent: web3.SYSVAR_RENT_PUBKEY,
                })
                .signers([vestingLockPdaKp])
                .rpc();

            console.log(`Vesting Lock Transaction: ${txHash}`);

            // Confirm transaction
            await provider.connection.confirmTransaction(txHash);

            // Fetch the created vesting lock PDA
            const vestingLockPdaAccount = await program.account.lockPda.fetch(vestingLockPdaKp.publicKey);

            console.log("Vesting Lock PDA data:", vestingLockPdaAccount);

            // Check if the lock amount is correct
            if (!vestingLockPdaAccount.lockAmount.eq(lockAmount)) {
                throw new Error("Vesting lock amount does not match");
            }
        } catch (error) {
            console.error("Error in vesting lock test:", error);
            throw error;
        }
    });

    it("should extend the lock time", async () => {
        const newLockTime = lockTime + 3600; // Extend by 1 hour

        try {
            const txHash = await program.methods
                .extendLockTime(new BN(newLockTime))
                .accounts({
                    lockPda: lockPdaKp.publicKey,
                    authority: provider.wallet.publicKey,
                    systemProgram: web3.SystemProgram.programId,
                })
                .rpc();

            console.log(`Extend Lock Time Transaction: ${txHash}`);

            // Confirm transaction
            await provider.connection.confirmTransaction(txHash);

            // Fetch the updated lock PDA
            const lockPdaAccount = await program.account.lockPda.fetch(lockPdaKp.publicKey);

            // Check if the lock time has been extended
            if (!lockPdaAccount.endTime.eq(new BN(newLockTime))) {
                throw new Error("Lock time was not extended correctly");
            }
        } catch (error) {
            console.error("Error in extend lock time test:", error);
            throw error;
        }
    });

    it("should handle error when trying to unlock before time", async () => {
        // Derive metadata PDA
        const metadataPda = await deriveMetadataPDA(projectTokenMint);

        try {
            const txHash = await program.methods
                .unlockToken(input)
                .accounts({
                    lockPda: lockPdaKp.publicKey,
                    authority: provider.wallet.publicKey,
                    splMint: projectTokenMint,
                    splMintMetadataPda: metadataPda,
                    lockPdaSplAta: lockPdaKp.publicKey,
                    authoritySplAta: provider.wallet.publicKey,
                })
                .rpc();

            // Confirm transaction
            await provider.connection.confirmTransaction(txHash);
            throw new Error("Expected an error to be thrown");
        } catch (error) {
            if (!error.message.includes("NotUnlockTime")) {
                console.error("Unexpected error:", error);
                throw error;
            }
        }
    });

    // Vesting unlock test (requires simulating time passing)
    it("should verify vesting unlock", async () => {
        // Derive metadata PDA
        const metadataPda = await deriveMetadataPDA(projectTokenMint);

        // Simulate time passing for vesting unlock
        const unlockTime = Math.floor(Date.now() / 1000) + 3600; // 1 hour from now

        try {
            const txHash = await program.methods
                .unlockVesting(input)
                .accounts({
                    lockPda: vestingLockPdaKp.publicKey,
                    authority: provider.wallet.publicKey,
                    splMint: projectTokenMint,
                    splMintMetadataPda: metadataPda,
                    lockPdaSplAta: vestingLockPdaKp.publicKey,
                    authoritySplAta: provider.wallet.publicKey,
                })
                .rpc();

            console.log(`Vesting Unlock Transaction: ${txHash}`);

            // Confirm transaction
            await provider.connection.confirmTransaction(txHash);

            // Fetch the updated vesting lock PDA
            const vestingLockPdaAccount = await program.account.lockPda.fetch(vestingLockPdaKp.publicKey);

            console.log("Vesting Lock PDA data:", vestingLockPdaAccount);

            // Check if the unlock amount is correct
            const expectedUnlockAmount = Math.floor((vestingLockPdaAccount.lockAmount.toNumber() * firstRelease) / 100);
            if (!vestingLockPdaAccount.lockAmount.eq(new BN(expectedUnlockAmount))) {
                throw new Error("Vesting unlock amount does not match");
            }
        } catch (error) {
            console.error("Error in vesting unlock test:", error);
            throw error;
        }
    });
});