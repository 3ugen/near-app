<script lang="ts">
  import {onMount} from "svelte"
  import NearApi from "../libs/near-api";
  import Attribute from "../components/Attribute.svelte";

  type ConnectionState = 'init' | 'disconnected' | 'connected'
  let isConnected: ConnectionState = 'init'

  const random = (min, max) => Math.floor(Math.random() * (max - min)) + min;

  let allKeys = "not set"
  let item_id = "not set"
  let model = "not set"
  let binance = 0
  let coinbase = 0
  let okx = 0
  let ftx = 0
  let kraken = 0

  let nearApi!: NearApi
  onMount(async () => {
    nearApi = await NearApi.init()
    checkConnection()
  })

  function checkConnection() {
    if (nearApi.isConnected()) {
      isConnected = 'connected'
    } else {
      isConnected = 'disconnected'
    }
    console.log(isConnected)
  }

  const walletConnect = async () => {
    await nearApi.walletConnect()
  }
  const walletDisconnect = () => {
    nearApi.walletDisconnect()
    checkConnection()
  }

  function getAccountName(): string {
    return nearApi.accountName()
  }

  const randomFill = () => {
    binance = random(8000, 20000)
    coinbase = random(8000, 20000)
    okx = random(8000, 20000)
    ftx = random(8000, 20000)
    kraken = random(8000, 20000)
  }

  const getAllKeys = async () => {
    const res = await nearApi.getKeys()
    console.log(`res: ${res}`)
    if (res.length > 0) {
      allKeys = res
    }
  }

  const getItem = async () => {
    const res = await nearApi.getItem(item_id)
    console.log(`res: ${res}`)
    if (res !== 'not found') {
      const val = JSON.parse(res) as {
        item_id: string,
        model: string,
        binance: number,
        coinbase: number,
        okx: number,
        ftx: number,
        kraken: number,
      }
      item_id = val.item_id
      model = val.model
      binance = val.binance
      coinbase = val.coinbase
      okx = val.okx
      ftx = val.ftx
      kraken = val.kraken
    }
  }
  const addItem = async () => {
    await nearApi.addItem(item_id,
      model,
      binance,
      coinbase,
      okx,
      ftx,
      kraken
    )
  }

</script>
<h1>Adv Oracle for Near protocol</h1>
<p>Connection state: {isConnected}</p>
{#if isConnected !== 'init'}
  {#if (isConnected === 'connected')}
    <p>Account: {getAccountName()}</p>
    <button on:click={walletDisconnect}>Disconnect</button>

  {:else}
    <button on:click={walletConnect}>Connect</button>
  {/if}
{/if}

<h3>All keys</h3>
{#if allKeys !== 'not set'}
  all keys: {allKeys}
{/if}<br/>
<button on:click={getAllKeys}>Get item keys</button>

{#if isConnected === 'connected'}
  <div class="form">
    <h3>Attribution models</h3>
    <button on:click={randomFill}>Random</button>
    <Attribute attrId='item_id' attrType='text' bind:value={item_id}/>
    <label for="advModel">Choose a model:</label>
    <select name="model" id="advModel">
      <option value="lastInteraction">Last Interaction</option>
      <option value="firstInteraction">First Interaction</option>
      <option value="markovChains">Markov Chains</option>
    </select>
    <Attribute attrId='Binance' attrType='number' bind:value={binance}/>
    <Attribute attrId='Coinbase' attrType='number' bind:value={coinbase}/>
    <Attribute attrId='Okx' attrType='number' bind:value={okx}/>
    <Attribute attrId='Ftx' attrType='number' bind:value={ftx}/>
    <Attribute attrId='Kraken' attrType='number' bind:value={kraken}/>
    <div style="display: flex; flex-direction: column; max-width: 10em; gap: 10px">
      <button on:click={getItem}>Get item</button>
      <button on:click={addItem}>Add items</button>
    </div>
  </div>
{/if}

<style>
  .form {
    display: flex;
    flex-direction: column;
    max-width: 300px;
    gap: 15px;
    margin-top: 30px
  }
</style>