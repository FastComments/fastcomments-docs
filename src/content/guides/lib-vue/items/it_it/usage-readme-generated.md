---
### Il widget dei commenti

Il componente FastCommentsVue contiene il widget dei commenti FastComments in tempo reale.

Sostituisci "demo" qui sotto con il tuo "tenantId" - disponibile [qui](https://fastcomments.com/auth/my-account/api) nell'area di amministrazione di FastComments.

Il widget supporta numerose opzioni - vedi FastCommentsConfig [qui](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts).

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