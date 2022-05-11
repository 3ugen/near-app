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
    tmpl
  } from '$stores/store';

  export let isConnected
  export let item_id

  export let randomFill
  export let getItem
  let modelOptions = [
    {id: 0, text: "Video Ads"},
    {id: 1, text: "Android Ads"},
    {id: 2, text: "iOS Ads"},
  ]
  let selected = modelOptions[0]

  const handleSelect = () => {
    tmpl.update(() => selected.text)
  }

  function formatStr(n: number): string {
    if (n < 10) {
      return '0' + n.toString()
    } else {
      return n.toString()
    }
  }

  const currentTime = () => {
    let dt = new Date()
    return `${formatStr(dt.getDate())}-${formatStr(dt.getMonth())}-${formatStr(dt.getFullYear())}`
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
          <select class="form-select block w-full mt-1" bind:value={selected} on:click={handleSelect}>
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
        <Attribute attrId='File Name' attrType='text' bind:value={$filename}/>
        <Attribute attrId='Tag Name' attrType='text' bind:value={$tagname}/>
        <Attribute attrId='Select a Title' attrType='text' bind:value={$title}/>
        <Attribute attrId='Publisher Name' attrType='text' bind:value={$pname}/>
        <Attribute attrId='Domain Name' attrType='text' bind:value={$gaming}/>
      </div>
    </div>
  </div>
{/if}