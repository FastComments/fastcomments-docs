---
### O widget de comentários

O componente FastCommentsVueNext contém o widget de comentários do FastComments em tempo real.

Substitua "demo" abaixo pelo seu "tenantId" - disponível [aqui](https://fastcomments.com/auth/my-account/api) na área de administração do FastComments.

O widget suporta muitas opções - veja FastCommentsConfig [aqui](https://github.com/FastComments/fastcomments-typescript/blob/eae973fb7885de4df58b21b7a22a3e40c89feefa/src/fastcomments-config.ts#L14).

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