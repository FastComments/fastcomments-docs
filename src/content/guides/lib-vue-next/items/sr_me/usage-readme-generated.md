---
### Vidžet za komentare

Komponenta FastCommentsVueNext sadrži uživo FastComments vidžet za komentare.

Zamijenite "demo" ispod sa svojim "tenantId" - dostupan [ovdje](https://fastcomments.com/auth/my-account/api) u FastComments administratorskom području.

Vidžet podržava mnogo opcija - pogledajte FastCommentsConfig [ovdje](https://github.com/FastComments/fastcomments-typescript/blob/eae973fb7885de4df58b21b7a22a3e40c89feefa/src/fastcomments-config.ts#L14).

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