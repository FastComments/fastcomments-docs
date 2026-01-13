### Kommentar-widgeten

FastCommentsVue-komponenten indeholder den live FastComments-kommentar-widget.

Erstat "demo" nedenfor med din "tenantId" - tilgængelig [her](https://fastcomments.com/auth/my-account/api) i FastComments administrationsområdet.

Widgeten understøtter mange muligheder - se FastCommentsConfig [her](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts).

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