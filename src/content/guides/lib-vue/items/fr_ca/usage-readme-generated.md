### Le widget de commentaires

Le composant FastCommentsVue contient le widget de commentaires FastComments en direct.

Remplacez "demo" ci-dessous par votre "tenantId" - disponible [ici](https://fastcomments.com/auth/my-account/api) dans l'espace d'administration FastComments.

Le widget prend en charge de nombreuses options - consultez FastCommentsConfig [ici](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts).

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