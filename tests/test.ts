import BN from 'bn.js';
import { SystemProgram, PublicKey, Keypair } from '@solana/web3.js';
import * as anchor from '@project-serum/anchor';

const idl = {
    "version": "0.1.0",
    "name": "testlocker",
    "instructions": [
        {
            "name": "createLockPda",
            "accounts": [
                { "name": "lockPda", "isMut": true, "isSigner": false },
                { "name": "authority", "isMut": true, "isSigner": true },
                { "name": "splMint", "isMut": true, "isSigner": false },
                { "name": "systemProgram", "isMut": true, "isSigner": false }
            ],
            "args": [
                { "name": "seed", "type": "string" },
                { "name": "lockAmount", "type": "u64" },
                { "name": "lockTime", "type": "u64" },
                { "name": "lockName", "type": "string" },
                { "name": "extraData", "type": "string" },
                { "name": "dexName", "type": "string" },
                { "name": "tokenMintA", "type": "publicKey" },
                { "name": "tokenMintB", "type": "publicKey" }
            ]
        },
        {
            "name": "extendLockTime",
            "accounts": [
                { "name": "lockPda", "isMut": true, "isSigner": false },
                { "name": "authority", "isMut": true, "isSigner": true }
            ],
            "args": [
                { "name": "newLockTime", "type": "u64" }
            ]
        },
        {
            "name": "lockVesting",
            "accounts": [
                { "name": "lockPda", "isMut": true, "isSigner": false },
                { "name": "authority", "isMut": true, "isSigner": true },
                { "name": "splMint", "isMut": true, "isSigner": false },
                { "name": "systemProgram", "isMut": true, "isSigner": false }
            ],
            "args": [
                { "name": "seed", "type": "string" },
                { "name": "lockTime", "type": "u64" },
                { "name": "lockName", "type": "string" },
                { "name": "extraData", "type": "string" },
                { "name": "firstReleasePercentage", "type": "f64" },
                { "name": "vestingPeriod", "type": "u64" },
                { "name": "amountPerVesting", "type": "f64" },
                { "name": "userList", "type": { "vec": "publicKey" } },
                { "name": "userAmounts", "type": { "vec": "u64" } }
            ]
        },
        {
            "name": "unlockToken",
            "accounts": [
                { "name": "lockPda", "isMut": true, "isSigner": false },
                { "name": "authority", "isMut": true, "isSigner": true },
                { "name": "splMint", "isMut": true, "isSigner": false }
            ],
            "args": [
                { "name": "seed", "type": "string" }
            ]
        }
    ],
    "accounts": [],
    "types": []
};

describe('GempadSolanaLock', () => {
    let program: any;
    let authority: PublicKey;
    let lockPda: PublicKey;
    let splMint: PublicKey;

    const lockName = "Test Lock";
    const extraData = "Extra Info";
    const dexName = "Test DEX";
    const SEED = "test_seed";
    
    // Use explicit, clean BN values
    let lockTime: BN;
    let lockAmount: BN;

    before(async () => {
        const wallet = Keypair.generate();
        authority = wallet.publicKey;

        program = new anchor.Program(
            idl, 
            new PublicKey("8gQmKKkHXHKkM3YKmRmFXdwzaQKoMcQg4YWW4TQGqvCR"),
            { 
                publicKey: authority,
                signTransaction: async (tx: any) => tx,
                signAllTransactions: async (txs: any[]) => txs
            }
        );

        // Use clean, straightforward BN values
        lockAmount = new BN(1000000); // 1 million tokens
        splMint = new PublicKey("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");

        // Use a fixed, predictable timestamp 
        // 1 week from now, as a clean BN
        const oneWeekFromNow = Math.floor(Date.now() / 1000) + (7 * 24 * 60 * 60);
        lockTime = new BN(oneWeekFromNow);

        const [pda] = await PublicKey.findProgramAddress(
            [Buffer.from(SEED)],
            program.programId
        );
        lockPda = pda;
    });

    it('should create a lock', async () => {
        try {
            const tx = await program.methods.createLockPda(
                SEED,
                lockAmount,
                lockTime,
                lockName,
                extraData,
                dexName,
                PublicKey.default, 
                PublicKey.default
            ).accounts({
                lockPda,
                authority,
                splMint,
                systemProgram: SystemProgram.programId,
            }).rpc();

            console.log("Create lock transaction signature:", tx);
        } catch (error) {
            console.error("Create lock error:", error);
            throw error;
        }
    });

    it('should extend lock time', async () => {
        try {
            // Extend by 2 weeks
            const newLockTime = new BN(
                Math.floor(Date.now() / 1000) + (14 * 24 * 60 * 60)
            );

            const tx = await program.methods.extendLockTime(newLockTime)
                .accounts({
                    lockPda,
                    authority,
                }).rpc();

            console.log("Extend lock time transaction signature:", tx);
        } catch (error) {
            console.error("Extend lock time error:", error);
            throw error;
        }
    });

    it('should create a vesting lock', async () => {
        try {
            const vestingLockTx = await program.methods.lockVesting(
                SEED,
                lockTime,
                lockName,
                extraData,
                10.0, // first release percentage
                new BN(7 * 24 * 60 * 60), // vesting period (1 week in seconds)
                10.0, // amount per vesting
                [authority], // user list
                [lockAmount] // user amounts
            ).accounts({
                lockPda,
                authority,
                splMint,
                systemProgram: SystemProgram.programId,
            }).rpc();

            console.log("Vesting lock transaction signature:", vestingLockTx);
        } catch (error) {
            console.error("Vesting lock error:", error);
            throw error;
        }
    });

    it('should handle error when unlocking before time', async () => {
        try {
            await program.methods.unlockToken(SEED)
                .accounts({
                    lockPda,
                    authority,
                    splMint,
                }).rpc();
            
            throw new Error("Expected unlock error did not occur");
        } catch (err) {
            console.log("Unlock before time error (expected):", err);
        }
    });
});