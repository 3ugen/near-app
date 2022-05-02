<script lang="ts">
  export let attrId: string
  export let attrType: 'text' | 'number'
  export let value: number
</script>
<div>
  {#if attrType === 'text'}
    <label for={attrId+'t'}>{attrId}:</label>
    <input type='text'
           id={attrId+'t'}
           name="name"
           value={value}
           on:keyup={e => value = e.target.value}
    >
  {:else}
    <label for={attrId+'n'}>{attrId}:</label>
    <input type='number'
           id={attrId+'n'}
           name="name"
           value={value}
           min=1000 max=20000
           on:keyup={e => value = e.target.value}
    >
  {/if}
</div>