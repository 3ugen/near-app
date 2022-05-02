import type {ConnectConfig, Contract, Near, WalletConnection} from "near-api-js";

type OracleContractT = Contract & {
    get_item(arg0: ArgsGet): Promise<string>,
    all_keys(): Promise<string>,
    add_item(arg0: ArgsAdd): Promise<void>,
}

class ArgsGet {
    item_id: string

    public constructor(item_id: string) {
        this.item_id = item_id;
    }
}

class ArgsAdd {
    item_id: string
    model: string
    binance: number
    coinbase: number
    okx: number
    ftx: number
    kraken: number

    public constructor(item_id: string,
                       model: string,
                       binance: number,
                       coinbase: number,
                       okx: number,
                       ftx: number,
                       kraken: number) {
        this.item_id = item_id
        this.model = model
        this.binance = binance
        this.coinbase = coinbase
        this.okx = okx
        this.ftx = ftx
        this.kraken = kraken
    }
}

export default class NearApi {
    private nearApi: any
    private near: Near
    private wallet!: WalletConnection
    private contract!: OracleContractT

    private constructor(nearApi: any, near: Near, wallet: WalletConnection, contract: OracleContractT) {
        this.nearApi = nearApi
        this.near = near
        this.wallet = wallet
        this.contract = contract
    }

    static async init() {
        await import('../utils/near-api-js.js')
        // eslint-disable-next-line  @typescript-eslint/no-explicit-any
        const nearApi = (window as any).nearApi as any
        console.log(nearApi)
        const config: ConnectConfig = {
            helperUrl: "https://helper.testnet.near.org",
            initialBalance: "",
            keyStore: undefined,
            masterAccount: "",
            networkId: "testnet",
            nodeUrl: "https://rpc.testnet.near.org",
            signer: undefined,
            walletUrl: "https://wallet.testnet.near.org",
            headers: {}
        }
        config.keyStore = new nearApi.keyStores.BrowserLocalStorageKeyStore()
        const near = await nearApi.connect(config) as Near
        console.log('connection state')
        console.table(near)
        const wallet = new nearApi.WalletConnection(near) as WalletConnection
        const contract: OracleContractT = new nearApi.Contract(
            wallet.account(),
            "advo1.liv1.testnet",
            {
                viewMethods: ["get_item", "all_keys"],
                changeMethods: ["add_item"],
                sender: wallet.account(),
            }
        )
        return new NearApi(nearApi, near, wallet, contract)
    }

    isConnected(): boolean {
        return !!this.wallet?.isSignedIn();
    }

    async walletConnect(): Promise<void> {
        await this.wallet?.requestSignIn({contractId: "laov1.liv1.testnet"}, 'oracle for Near protocol')
    }

    walletDisconnect() {
        this.wallet?.signOut()
    }

    accountName(): string {
        return this.wallet?.getAccountId() as string
    }

    async getItem(item_id: string): Promise<string> {
        return await this.contract?.get_item({item_id})
    }

    async getKeys(): Promise<string> {
        return await this.contract?.all_keys()
    }

    async addItem(item_id: string,
                  model: string,
                  binance: number,
                  coinbase: number,
                  okx: number,
                  ftx: number,
                  kraken: number,
    ): Promise<void> {
        console.log('add item')
        return await this.contract?.add_item({
            item_id,
            model,
            binance,
            coinbase,
            okx,
            ftx,
            kraken,
        })
    }
}