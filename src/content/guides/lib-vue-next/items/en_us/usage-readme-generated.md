### The Comment Widget

The FastCommentsVueNext component contains the live FastComments comment widget.

Replace "demo" below with your "tenantId" — available [here](https://fastcomments.com/auth/my-account/api) in the FastComments admin area.

The widget supports many options — see FastCommentsConfig [here](https://github.com/FastComments/fastcomments-typescript/blob/eae973fb7885de4df58b21b7a22a3e40c89feefa/src/fastcomments-config.ts#L14).

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