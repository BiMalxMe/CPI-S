import {test,expect} from "bun:test"
import { LiteSVM } from "litesvm";
import {
	PublicKey,
	Transaction,
	SystemProgram,
	Keypair,
	LAMPORTS_PER_SOL,
    TransactionInstruction,
} from "@solana/web3.js";
import { sys } from "typescript";

test("one transfer", () => {
	const svm = new LiteSVM();
    //we dont need a secret ley so just creating a publik key only
    const contractpubkey = PublicKey.unique()
    svm.addProgramFromFile(contractpubkey,"./cpi.so")
	const payer = new Keypair();
	svm.airdrop(payer.publicKey, BigInt(LAMPORTS_PER_SOL));
	const dataAccount = Keypair.generate();
	const blockhash = svm.latestBlockhash();
	const ixs = [
		SystemProgram.createAccount({
			fromPubkey: payer.publicKey,
			newAccountPubkey: dataAccount.publicKey,
			lamports: Number( svm.minimumBalanceForRentExemption(BigInt(4))),
            space :4,
            programId : contractpubkey
		}),
	];
	const tx = new Transaction();
	tx.recentBlockhash = blockhash;
	tx.add(...ixs);
	tx.sign(payer,dataAccount);
	svm.sendTransaction(tx);
	const balanceAfter = svm.getBalance(dataAccount.publicKey);
	expect(balanceAfter).toBe(svm.minimumBalanceForRentExemption(BigInt(4)));

    function doubledata(){
    const ix2 = new TransactionInstruction({
        keys : [
            {pubkey:dataAccount.publicKey,isSigner:false,isWritable:true}
        ],
        programId : contractpubkey,
        data : Buffer.from("")
    })
    const tx2 = new Transaction();
    const blockhash = svm.latestBlockhash()
	tx2.recentBlockhash = blockhash;
    tx2.feePayer = payer.publicKey;
	tx2.add(ix2);
	tx2.sign(payer);
	svm.sendTransaction(tx2);
    svm.expireBlockhash()
}
doubledata()
doubledata()
doubledata()

console.log(dataAccount.publicKey)
const  d = svm.getAccount(dataAccount.publicKey)
console.log(d)
expect(d?.data[0]).toBe(4)
expect(d?.data[1]).toBe(0)
expect(d?.data[2]).toBe(0)
expect(d?.data[3]).toBe(0)

});