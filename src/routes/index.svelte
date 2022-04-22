<script>
  import {onMount} from "svelte"

  let nearState = 'undefined'
  let nearWallet;

  onMount(async () => {
    // setup some globals
    import('../utils/near-api-js.js').then((window) => {
        const keyStore = new nearApi.keyStores.BrowserLocalStorageKeyStore();
        const config = {
          networkId: "testnet",
          keyStore: keyStore,
          nodeUrl: "https://rpc.testnet.near.org",
          walletUrl: "https://wallet.testnet.near.org",
          helperUrl: "https://helper.testnet.near.org",
          explorerUrl: "https://explorer.testnet.near.org",
        };
        nearApi.connect(config).then(res => {
          console.log('connection state')
          console.table(res)
          const wallet = new nearApi.WalletConnection(res);
          if (wallet.isConnected) {
            nearState = 'connected'
          } else {
            nearState = 'disconnected'
          }
          nearWallet = wallet
        });

      }
    )
  })

  async function nearConnect() {
    nearWallet.requestSignIn(
      "example-contract.testnet", // contract requesting access
      "Example App", // optional
      "http://localhost:3000", // optional
      "http://localhost:3000", // optional
    )
  }

</script>
<h1>Welcome to Near</h1>
<p>Near connection state: {nearState}</p>
<button on:click={nearConnect}>connect to Near</button>