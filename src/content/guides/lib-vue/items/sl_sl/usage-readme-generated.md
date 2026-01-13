### Pripomoček za komentarje

Komponenta FastCommentsVue vsebuje živi FastComments pripomoček za komentarje.

Zamenjajte "demo" spodaj z vašim "tenantId" - na voljo [tukaj](https://fastcomments.com/auth/my-account/api) v skrbniškem območju FastComments.

Pripomoček podpira veliko možnosti - oglejte si FastCommentsConfig [tukaj](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts).

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