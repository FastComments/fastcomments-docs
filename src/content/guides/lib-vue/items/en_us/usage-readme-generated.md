### The Comment Widget

The FastCommentsVue component contains the live FastComments comment widget.

Replace "demo" below with your "tenantId" - available [here](https://fastcomments.com/auth/my-account/api) in the FastComments admin area.

The widget supports a lot of options - see FastCommentsConfig [here](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts).

```vue
<script lang="ts">
import Vue from 'vue';
import FastCommentsVue from 'fastcomments-vue';

export default Vue.extend({
  name: 'ServeDev',
  components: {
    FastCommentsVue
  }
});
</script>

<template>
  <div id="app">
    <fast-comments-vue v-bind:config="{tenantId: 'demo'}" />
  </div>
</template>
```