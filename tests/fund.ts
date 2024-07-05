import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Fund } from "../target/types/fund";
import { expect } from "chai";
import { getAssociatedTokenAddress, getAccount } from "@solana/spl-token";

describe("fund", () => {
    const provider = anchor.AnchorProvider.env();

    anchor.setProvider(provider);

    const program = anchor.workspace.Fund as Program<Fund>;
    const connection = provider.connection;

    const manager = anchor.web3.Keypair.generate();

    const [fundPDA] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("fund")],
        program.programId
    );

    const [vaultPDA] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("fund_vault")],
        program.programId
    );

    const fundData = {
        initialInvestment: 2_000_000,
        initialShares: 10_000
    };

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

        const vaultAccount = await getAccount(
            connection,
            vaultPDA
        );

        expect(Number(fundAccount.totalValue)).to.equal(fundData.initialInvestment);
        expect(Number(fundAccount.totalShares)).to.equal(fundData.initialShares);

        expect(Number(vaultAccount.amount)).to.equal(fundData.initialShares);
    });

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

    it("Pays an invesmtent and updates total value", async () => {

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




});
