### Widget za komentare

Komponenta FastCommentsVueNext sadrži uživo FastComments widget za komentare.

Zamenite "demo" ispod sa vašim "tenantId" - dostupan [ovde](https://fastcomments.com/auth/my-account/api) u FastComments administratorskom delu.

Widget podržava mnogo opcija - pogledajte FastCommentsConfig [ovde](https://github.com/FastComments/fastcomments-typescript/blob/eae973fb7885de4df58b21b7a22a3e40c89feefa/src/fastcomments-config.ts#L14).

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