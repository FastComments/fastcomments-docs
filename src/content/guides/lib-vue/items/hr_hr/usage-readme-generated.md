### Widget komentara

Komponenta FastCommentsVue sadrži uživo FastComments widget za komentare.

Zamijenite "demo" u nastavku sa svojim "tenantId" - dostupan [ovdje](https://fastcomments.com/auth/my-account/api) u FastComments administratorskom području.

Widget podržava mnoge opcije - pogledajte FastCommentsConfig [ovdje](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts).

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