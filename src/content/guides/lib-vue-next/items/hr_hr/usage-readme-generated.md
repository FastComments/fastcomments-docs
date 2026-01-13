---
### Widget komentara

Komponenta FastCommentsVueNext sadrži FastComments widget za komentare uživo.

Zamijenite "demo" dolje sa svojim "tenantId" - dostupan [ovdje](https://fastcomments.com/auth/my-account/api) u administratorskom području FastComments.

Widget podržava mnogo opcija - pogledajte FastCommentsConfig [ovdje](https://github.com/FastComments/fastcomments-typescript/blob/eae973fb7885de4df58b21b7a22a3e40c89feefa/src/fastcomments-config.ts#L14).

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