### Yorum Bileşeni

FastCommentsVue bileşeni canlı FastComments yorum widget'ını içerir.

Aşağıdaki "demo"yu FastComments yönetici alanında [buradan](https://fastcomments.com/auth/my-account/api) bulunan "tenantId"nizle değiştirin.

Widget birçok seçeneği destekler - FastCommentsConfig'e [buradan](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts) bakın.

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