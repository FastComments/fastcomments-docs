### Il widget dei commenti

Il componente FastCommentsVueNext contiene il widget dei commenti FastComments in tempo reale.

Sostituisci "demo" di seguito con il tuo "tenantId" - disponibile [qui](https://fastcomments.com/auth/my-account/api) nell'area di amministrazione di FastComments.

Il widget supporta numerose opzioni - vedi FastCommentsConfig [qui](https://github.com/FastComments/fastcomments-typescript/blob/eae973fb7885de4df58b21b7a22a3e40c89feefa/src/fastcomments-config.ts#L14).

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