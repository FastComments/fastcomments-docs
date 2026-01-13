### Le widget de commentaires

Le composant FastCommentsVueNext contient le widget de commentaires FastComments en direct.

Remplacez "demo" ci-dessous par votre "tenantId" - disponible [ici](https://fastcomments.com/auth/my-account/api) dans la zone d'administration FastComments.

Le widget prend en charge de nombreuses options - consultez FastCommentsConfig [ici](https://github.com/FastComments/fastcomments-typescript/blob/eae973fb7885de4df58b21b7a22a3e40c89feefa/src/fastcomments-config.ts#L14).

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