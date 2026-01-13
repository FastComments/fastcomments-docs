### El widget de comentarios

El componente FastCommentsVue contiene el widget de comentarios en vivo de FastComments.

Reemplaza "demo" abajo por tu "tenantId" - disponible [aquí](https://fastcomments.com/auth/my-account/api) en el área de administración de FastComments.

El widget admite muchas opciones - consulta FastCommentsConfig [aquí](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts).

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