<script lang="ts">
  import {onMount} from "svelte"
  import NearApi from "../libs/near-api";
  import Attribute from "../components/Attribute.svelte";

  type ConnectionState = 'init' | 'disconnected' | 'connected'
  let isConnected: ConnectionState = 'init'

  let item_id = "not set"
  let direct = 0
  let email = 0
  let fb = 0
  let g_search = 0
  let organic = 0
  let youtube = 0

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

  const getItem = async () => {
    const res = await nearApi.getItem(item_id)
    console.log(`res: ${res}`)
    if (res !== 'not found') {
      const val = JSON.parse(res) as {
        item_id: string,
        direct: number,
        email: number,
        fb: number,
        g_search: number,
        organic: number,
        youtube: number
      }
      item_id = val.item_id
      direct = val.direct
      email = val.email
      fb = val.fb
      g_search = val.g_search
      organic = val.organic
      youtube = val.youtube
    }
  }
  const addItem = async () => {
    await nearApi.addItem(item_id,
      direct,
      email,
      fb,
      g_search,
      organic,
      youtube
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

{#if isConnected === 'connected'}
  <div class="form">
    <h3>Attribution models</h3>
    <Attribute attrId='item_id' attrType='text' bind:value={item_id}/>
    <Attribute attrId='direct' attrType='number' bind:value={direct}/>
    <Attribute attrId='email' attrType='number' bind:value={email}/>
    <Attribute attrId='fb' attrType='number' bind:value={fb}/>
    <Attribute attrId='g_search' attrType='number' bind:value={g_search}/>
    <Attribute attrId='organic' attrType='number' bind:value={organic}/>
    <Attribute attrId='youtube' attrType='number' bind:value={youtube}/>
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