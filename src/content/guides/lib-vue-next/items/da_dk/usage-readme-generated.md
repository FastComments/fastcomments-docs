### Kommentar-widgeten

FastCommentsVueNext-komponenten indeholder den live FastComments kommentar-widget.

Erstat "demo" nedenfor med din "tenantId" - tilgængelig [her](https://fastcomments.com/auth/my-account/api) i FastComments' administrationsområde.

Widgeten understøtter mange indstillinger - se FastCommentsConfig [her](https://github.com/FastComments/fastcomments-typescript/blob/eae973fb7885de4df58b21b7a22a3e40c89feefa/src/fastcomments-config.ts#L14).

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