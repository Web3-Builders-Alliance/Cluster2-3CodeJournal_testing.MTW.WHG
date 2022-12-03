import { SigningCosmWasmClient, Secp256k1HdWallet, GasPrice, Coin } from "cosmwasm";

import * as fs from 'fs';
import axios from 'axios';
import { ClientRequest } from "http";

const rpcEndpoint = "https://rpc.uni.juno.deuslabs.fi";

const messages_wasm = fs.readFileSync("../artifacts/messages.wasm");

const mnemonic =
    "test peanut elevator motor proud globe obtain gasp sad balance nature ladder";

const code_id = 2509;

const contract_address = "juno1weqt9ksm9k8yq2ekvxlae9jday76azaywnd65p4d9n8gppcura9svq38vv";

async function setupClient(mnemonic: string, rpc: string, gas: string | undefined): Promise<SigningCosmWasmClient> {
    if (gas === undefined) {
        let wallet = await Secp256k1HdWallet.fromMnemonic(mnemonic, { prefix: 'juno'});
        let client = await SigningCosmWasmClient.connectWithSigner(rpc, wallet);
        return client;
    } else {
        let gas_price = GasPrice.fromString(gas);
        let wallet = await Secp256k1HdWallet.fromMnemonic(mnemonic, { prefix: 'juno' });
        let client = await SigningCosmWasmClient.connectWithSigner(rpc, wallet, { gasPrice: gas_price });
        return client;
    }
}

async function getAddress(mnemonic: string, prefix: string = 'juno') {
    let wallet = await Secp256k1HdWallet.fromMnemonic(mnemonic, { prefix });
    let accounts = await wallet.getAccounts();
    return accounts[0].address;
}

describe("Messages Fullstack Test", () => {
    xit("Generate Wallet", async () => {
        let wallet = await Secp256k1HdWallet.generate(12);
        console.log(wallet.mnemonic);
    });

    xit("Get Testnet Tokens", async () => {
        //let wallet = await Secp256k1HdWallet.fromMnemonic(mnemonic, { prefix: 'juno' });
        //console.log(await wallet.getAccounts());
        console.log(await getAddress(mnemonic));
        try {
            let res = await axios.post("https://faucet.uni.juno.deuslabs.fi/credit", { "denom": "ujunox", "address": await getAddress(mnemonic) });
            console.log(res);
        } catch (e) {
            console.log(e);
        }
    }).timeout(100000);

    xit("Send Testnet Tokens", async () => {
        let client = await setupClient(mnemonic, rpcEndpoint, "0.025ujunox");
        let receiver = "";
        let res = await client.sendTokens(await getAddress(mnemonic), receiver, [{denom:"ujunox", amount:"1000000"}], "auto");
        console.log(res);
    }).timeout(100000);

    //same as
    //junod tx wasm store artifacts/messages.wasm --from wallet --node https://rpc.uni.juno.deuslabs.fi --chain_id=uni-3 --gas-price=0.025ujunox --gas auto
    xit("Upload code to testnet", async () => {
        let client = await setupClient(mnemonic, rpcEndpoint, "0.025ujunox");
        let res = await client.upload(await getAddress(mnemonic), messages_wasm, "auto");
        //calculateFee()
        console.log(JSON.stringify(res.logs[0].events));
    }).timeout(100000);

    xit("Instantiate code on testnet", async () => {
        let client = await setupClient(mnemonic, rpcEndpoint, "0.025ujunox");
        let res = await client.instantiate(await getAddress(mnemonic), code_id, { }, "messages", "auto");
        console.log(res);
    }).timeout(100000);

    xit("Add Message on testnet", async() => {
        let client = await setupClient(mnemonic, rpcEndpoint, "0.025ujunox");
        let res = await client.execute(await getAddress(mnemonic), contract_address, { add_message: { message: "bla bla", topic: "topic"}}, "auto");
        console.log(res);
    }).timeout(20000);

    xit("Query all messages on testnet", async () => {
        
    }).timeout(50000);

    xit("Query get all messages by addr", async() => {
        let client = await setupClient(mnemonic, rpcEndpoint, "0.025ujunox");
        let res = await client.queryContractSmart(contract_address, { get_messages_by_addr: { address: "" } } );
        console.log(res);
    }).timeout(100000);

    xit("Query get current id", async() => {

    }).timeout(100000);

    xit("Query get messages by topic", async() => {

    }).timeout(100000);
});