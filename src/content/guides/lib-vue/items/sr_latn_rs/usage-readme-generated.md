---
### Widget za komentare

Komponenta FastCommentsVue sadrži widget za komentare FastComments uživo.

Zamenite "demo" ispod sa svojim "tenantId" - dostupan je [ovde](https://fastcomments.com/auth/my-account/api) u administratorskom delu FastComments-a.

Widget podržava mnogo opcija - pogledajte FastCommentsConfig [ovde](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts).

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