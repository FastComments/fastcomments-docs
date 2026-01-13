### El widget de comentarios

El componente FastCommentsVueNext contiene el widget de comentarios FastComments en vivo.

Reemplaza "demo" abajo por tu "tenantId" - disponible [aquí](https://fastcomments.com/auth/my-account/api) en el área de administración de FastComments.

El widget admite muchas opciones - consulta FastCommentsConfig [aquí](https://github.com/FastComments/fastcomments-typescript/blob/eae973fb7885de4df58b21b7a22a3e40c89feefa/src/fastcomments-config.ts#L14).

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