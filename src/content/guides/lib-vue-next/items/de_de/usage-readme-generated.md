### Das Kommentar-Widget

Die Komponente FastCommentsVueNext enthält das Live-Kommentar-Widget von FastComments.

Ersetze "demo" unten durch deine "tenantId" - verfügbar [hier](https://fastcomments.com/auth/my-account/api) im Admin-Bereich von FastComments.

Das Widget unterstützt viele Optionen - siehe FastCommentsConfig [hier](https://github.com/FastComments/fastcomments-typescript/blob/eae973fb7885de4df58b21b7a22a3e40c89feefa/src/fastcomments-config.ts#L14).

```vue
<template>
  <FastComments v-bind:config="{tenantId: 'demo'}" />
</template>
<script>
import { FastComments } from 'fastcomments-vue-next'
export default {
  name: 'FastCommentsExample',
  components: {
    FastComments
  }
}
</script>
```