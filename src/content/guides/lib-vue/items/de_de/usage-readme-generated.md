### Das Kommentar-Widget

Die FastCommentsVue-Komponente enthält das Live-FastComments-Kommentar-Widget.

Ersetzen Sie "demo" unten durch Ihre "tenantId" - verfügbar [hier](https://fastcomments.com/auth/my-account/api) im FastComments-Adminbereich.

Das Widget unterstützt viele Optionen - siehe FastCommentsConfig [hier](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts).

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