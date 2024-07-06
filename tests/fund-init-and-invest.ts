import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Fund } from "../target/types/fund";
import { expect } from "chai";
import { getAccount, createMint, mintTo, getAssociatedTokenAddress, getOrCreateAssociatedTokenAccount } from "@solana/spl-token";

describe("fund init and invest", () => {
    const provider = anchor.AnchorProvider.env();

    anchor.setProvider(provider);

    const program = anchor.workspace.Fund as Program<Fund>;
    const connection = provider.connection;

    const manager = anchor.web3.Keypair.generate();

    // FOR INITIALIZING THE FUND

    const [fundPDA] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("fund")],
        program.programId
    );

    const [fundMintPDA] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("mint")],
        program.programId
    );

    const [sharesVaultPDA] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("fund_shares_vault")],
        program.programId
    );

    const fundData = {
        initialInvestment: 2_000_000,
        initialShares: 10_000
    };

    // FOR INITIALIZING THE "USDC" MINT AND VAULT

    let USDC_MINT: anchor.web3.PublicKey;

    const [usdcVauldPDA] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("fund_usdc_vault")],
        program.programId
    );

    // FOR TESTING INVESTMENTS

    const investment = {
        investmentAmount: 500_000,
        paymentAmount: 550_000,
        maturityDate: 100000,
        paymentDate: 100001,
        identifier: "prueba"
    };
    
    const [investmentPDA] = anchor.web3.PublicKey.findProgramAddressSync(
        [
            Buffer.from("investment"),
            Buffer.from(investment.identifier),
        ],
        program.programId
    );

    const badInvestment = {
        investmentAmount: 500_000,
        paymentAmount: 450_000,
        maturityDate: 100000,
        paymentDate: 100001,
        identifier: "prueba1"
    };

    const [badInvestmentPDA] = anchor.web3.PublicKey.findProgramAddressSync(
        [
            Buffer.from("investment"),
            Buffer.from(badInvestment.identifier),
        ],
        program.programId
    );

    // FOR TESTING USER BUYS SHARES

    const investor = anchor.web3.Keypair.generate();
    

    it("Initializes the fund and mints the tokens", async () => {
        
        const tx = await connection.requestAirdrop(
            manager.publicKey, 
            anchor.web3.LAMPORTS_PER_SOL * 10
        );
    
        await connection.confirmTransaction(tx);

        await program.methods
            .initializeFund(
                new anchor.BN(fundData.initialInvestment), 
                new anchor.BN(fundData.initialShares)
            )
            .accounts({
                manager: manager.publicKey,
            })
            .signers([manager])
            .rpc();

        const fundAccount = await program.account.fundAccount.fetch(fundPDA);

        const shareVaultAccount = await getAccount(
            connection,
            sharesVaultPDA
        );

        expect(Number(fundAccount.totalValue)).to.equal(fundData.initialInvestment);
        expect(Number(fundAccount.totalShares)).to.equal(fundData.initialShares);

        expect(Number(shareVaultAccount.amount)).to.equal(fundData.initialShares);
    });

    it("Initializes the USDC vault account", async () => {

        USDC_MINT = await createMint(
            connection,
            manager,
            manager.publicKey,
            null,
            0
        );

        await program.methods
            .initializeUsdcVault()
            .accounts({
                usdcMint: USDC_MINT,
                manager: manager.publicKey
            })
            .signers([manager])
            .rpc();

        const fundAccount = await program.account.fundAccount.fetch(fundPDA);

        const usdcVaultAccount = await getAccount(
            connection,
            usdcVauldPDA
        );

        expect(fundAccount.usdcVault.toString()).to.equal(usdcVauldPDA.toString());
        expect(fundAccount.usdcVault.toString()).to.equal(usdcVaultAccount.address.toString());
        expect(Number(usdcVaultAccount.amount)).to.equal(0);
    })

    it("Adds an investment to the fund and updates total value", async () => {

        await program.methods
            .addInvestment(
                investment.identifier,
                new anchor.BN(investment.investmentAmount),
                new anchor.BN(investment.maturityDate)
            )
            .accounts({
                manager: manager.publicKey,
            })
            .signers([manager])
            .rpc();

        const fundAccount = await program.account.fundAccount.fetch(fundPDA);
        const investmentAccount = await program.account.investmentAccount.fetch(investmentPDA);

        expect(Number(fundAccount.totalValue)).to.equal(2_500_000);

        expect(Number(investmentAccount.investmentAmount)).to.equal(investment.investmentAmount);
        expect(Number(investmentAccount.maturityDate)).to.equal(investment.maturityDate);
        expect(investmentAccount.identifier).to.equal(investment.identifier);
    });

    it("Pays an investment and updates total value and share value", async () => {

        await program.methods
            .payInvestment(
                investment.identifier,
                new anchor.BN(investment.paymentAmount),
                new anchor.BN(investment.paymentDate)
            )
            .accounts({
                manager: manager.publicKey,
            })
            .signers([manager])
            .rpc();

        const fundAccount = await program.account.fundAccount.fetch(fundPDA);
        const investmentAccount = await program.account.investmentAccount.fetch(investmentPDA);

        expect(Number(fundAccount.totalValue)).to.equal(2_550_000);

        expect(Number(investmentAccount.paymentAmount)).to.equal(investment.paymentAmount);
        expect(Number(investmentAccount.paymentDate)).to.equal(investment.paymentDate);
        expect(investmentAccount.identifier).to.equal(investment.identifier);
    });

    it("Pays an investment and updates total value and share value when investment return was negative", async () => {

        await program.methods
            .addInvestment(
                badInvestment.identifier,
                new anchor.BN(badInvestment.investmentAmount),
                new anchor.BN(badInvestment.maturityDate)
            )
            .accounts({
                manager: manager.publicKey,
            })
            .signers([manager])
            .rpc();

        let fundAccount = await program.account.fundAccount.fetch(fundPDA);

        const fundTotalValueBeforePay = Number(fundAccount.totalValue);

        expect(fundTotalValueBeforePay).to.equal(2_550_000 + badInvestment.investmentAmount);


        await program.methods
            .payInvestment(
                badInvestment.identifier,
                new anchor.BN(badInvestment.paymentAmount),
                new anchor.BN(badInvestment.paymentDate)
            )
            .accounts({
                manager: manager.publicKey,
            })
            .signers([manager])
            .rpc();

        const badInvestmentAccount = await program.account.investmentAccount.fetch(badInvestmentPDA);
        fundAccount = await program.account.fundAccount.fetch(fundPDA);

        const badInvestmentBalance = Number(badInvestmentAccount.paymentAmount) - Number(badInvestmentAccount.investmentAmount);

        expect(Number(fundAccount.totalValue)).to.equal(fundTotalValueBeforePay + badInvestmentBalance);
    });

    it("User can invest with USDC tokens, gets share tokens, fund USDC vault balance increases", async () => {
        const tx = await connection.requestAirdrop(
            investor.publicKey, 
            anchor.web3.LAMPORTS_PER_SOL * 10
        );
    
        await connection.confirmTransaction(tx);

        const investorUsdcATA = await getOrCreateAssociatedTokenAccount(
            connection,
            investor,
            USDC_MINT,
            investor.publicKey
        );

        await mintTo(
            connection,
            investor,
            USDC_MINT,
            investorUsdcATA.address,
            manager,
            500000
        );

        await program.methods
            .buyShares(new anchor.BN(50000))
            .accounts({
                usdcMint: USDC_MINT,
                buyerUsdcTokenAccount: investorUsdcATA.address,
                buyer: investor.publicKey
            })
            .signers([investor])
            .rpc();
            
        const fundAccount = await program.account.fundAccount.fetch(fundPDA);

        const userFundATA = await getAssociatedTokenAddress(
            fundMintPDA,
            investor.publicKey,
        );

        const userfundATAAccount = await getAccount(
            connection,
            userFundATA
        );

        const fundUsdcVault = await getAccount(
            connection,
            usdcVauldPDA
        );

        expect(Number(userfundATAAccount.amount)).to.equal(166);
        expect(Number(fundAccount.totalShares)).to.equal(10166);
        expect(Number(fundAccount.totalValue)).to.equal(3_050_000);
        expect(Number(fundUsdcVault.amount)).to.equal(50000);
    })
});
