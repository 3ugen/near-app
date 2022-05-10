<script lang="ts">
  import Attribute from "./Attribute.svelte";
  import {
    adname,
    bidprice,
    cryptotype,
    filename,
    tagname,
    title,
    pname,
    gaming,
  } from '$stores/store';

  export let isConnected
  export let item_id

  export let randomFill
  export let getItem
  export let addItem
  export let addItemRubicon
  export let getItemRubicon
  let modelOptions = [
    {id: 0, text: "Video Ads"},
    {id: 1, text: "Android Ads"},
    {id: 2, text: "iOS Ads"},
  ]
  let selected = modelOptions[0]
  const currentTime = () => {
    let dt = new Date()
    return `${dt.getHours()}:${dt.getMinutes()}`
  };
</script>
{#if isConnected === 'connected'}
  <div class="py-12">
    <h3 class="text-2xl font-bold">Real-time Bid Template(RTB) Portal</h3>
    <p>Logged in : lawrence@gmail.com</p>
    <p>Current Date : {currentTime()}</p>
    <div class="mt-8 max-w-md">
      <div class="grid grid-cols-1 gap-6">
        <Attribute attrId='item_id' attrType='text' bind:value={item_id}/>
        <Attribute attrId='Advertisor Name' attrType='text' bind:value={$adname}/>
        <label class="block">
          <span class="text-gray-700">Template option:</span>
          <select class="form-select block w-full mt-1" bind:value={selected}>
            <option value={modelOptions[0]}>
              {modelOptions[0].text}
            </option>
            <option value={modelOptions[1]}>
              {modelOptions[1].text}
            </option>
            <option value={modelOptions[2]}>
              {modelOptions[2].text}
            </option>
            <!--{#each modelOptions as model}
              <option value={model}>
                {model.text}
              </option>
            {/each}-->
          </select>
        </label>
        <Attribute attrId='Minimum Bid Floor' attrType='number' bind:value={$bidprice}/>
        <Attribute attrId='Crypto Protocol' attrType='text' bind:value={$cryptotype}/>
        <Attribute attrId='File Name' attrType='text' bind:value={$tagname}/>
        <Attribute attrId='Tag Name' attrType='text' bind:value={$title}/>
        <Attribute attrId='Select a Title' attrType='text' bind:value={$pname}/>
        <Attribute attrId='Publisher Name' attrType='text' bind:value={$gaming}/>
        <Attribute attrId='Domain Name' attrType='text' bind:value={$gaming}/>

      </div>

    </div>
    <div class="py-4 px-2 flex flex-row justify-between">
      <!--<button class="btn-primary" on:click={getItem}>Get item</button>
      <button class="{(item_id.length === 0 || item_id === 'not set') ? 'btn-disabled' : 'btn-primary'}"
              on:click={addItem}>Add items
      </button>-->

      <button class="btn-primary" on:click={getItem}>Submit the Bid</button>
      <button class="btn-primary" on:click={addItemRubicon}>Generate JSON Request</button>
      <button class="btn-primary" on:click={getItemRubicon}>Generate JSON Response</button>
    </div>
  </div>
{/if}