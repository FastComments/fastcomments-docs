---
### O Widget de Comentários

O componente FastCommentsVue contém o widget de comentários do FastComments em tempo real.

Substitua "demo" abaixo pelo seu "tenantId" - disponível [aqui](https://fastcomments.com/auth/my-account/api) na área administrativa do FastComments.

O widget suporta muitas opções - veja FastCommentsConfig [aqui](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts).

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