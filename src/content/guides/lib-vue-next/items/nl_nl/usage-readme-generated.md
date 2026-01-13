### De Reactie-widget

De FastCommentsVueNext-component bevat de live FastComments-reactie-widget.

Vervang hieronder "demo" door uw "tenantId" - beschikbaar [hier](https://fastcomments.com/auth/my-account/api) in het FastComments-beheergebied.

De widget ondersteunt veel opties - zie FastCommentsConfig [hier](https://github.com/FastComments/fastcomments-typescript/blob/eae973fb7885de4df58b21b7a22a3e40c89feefa/src/fastcomments-config.ts#L14).

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