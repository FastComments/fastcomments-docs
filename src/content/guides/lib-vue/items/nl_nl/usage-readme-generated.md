---
### De Reactie-widget

De FastCommentsVue-component bevat de live FastComments-reactie-widget.

Vervang "demo" hieronder door uw "tenantId" - beschikbaar [hier](https://fastcomments.com/auth/my-account/api) in het FastComments-beheergebied.

De widget ondersteunt veel opties - zie FastCommentsConfig [hier](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts).

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
---