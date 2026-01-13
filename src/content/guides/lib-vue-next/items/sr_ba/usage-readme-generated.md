---
### Видгет за коментаре

Компонента FastCommentsVueNext садржи FastComments видгет за коментаре у реалном времену.

Замените "demo" доле са вашим "tenantId" - доступно [овдје](https://fastcomments.com/auth/my-account/api) у админ подручју FastComments.

Видгет подржава много опција - погледајте FastCommentsConfig [овдје](https://github.com/FastComments/fastcomments-typescript/blob/eae973fb7885de4df58b21b7a22a3e40c89feefa/src/fastcomments-config.ts#L14).

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
---