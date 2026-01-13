### Уиджетът за коментари

Компонентът FastCommentsVueNext съдържа живия уиджет за коментари на FastComments.

Заменете "demo" по-долу с вашия "tenantId" - наличен [тук](https://fastcomments.com/auth/my-account/api) в администраторския панел на FastComments.

Уиджетът поддържа много опции - вижте FastCommentsConfig [тук](https://github.com/FastComments/fastcomments-typescript/blob/eae973fb7885de4df58b21b7a22a3e40c89feefa/src/fastcomments-config.ts#L14).

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