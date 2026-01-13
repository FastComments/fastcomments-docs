---
### Коментар видгет

Компонента FastCommentsVueNext садржи видгет FastComments за коментаре у реалном времену.

Замените "demo" доле са вашим "tenantId" - доступно [овде](https://fastcomments.com/auth/my-account/api) у FastComments админ подручју.

Видгет подржава много опција - погледајте FastCommentsConfig [овде](https://github.com/FastComments/fastcomments-typescript/blob/eae973fb7885de4df58b21b7a22a3e40c89feefa/src/fastcomments-config.ts#L14).

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