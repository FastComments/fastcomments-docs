---
### Το Widget Σχολίων

Το συστατικό FastCommentsVueNext περιέχει το ζωντανό widget σχολίων FastComments.

Αντικαταστήστε το "demo" παρακάτω με το "tenantId" σας - διαθέσιμο [εδώ](https://fastcomments.com/auth/my-account/api) στην περιοχή διαχείρισης FastComments.

Το widget υποστηρίζει πολλές επιλογές - δείτε το FastCommentsConfig [εδώ](https://github.com/FastComments/fastcomments-typescript/blob/eae973fb7885de4df58b21b7a22a3e40c89feefa/src/fastcomments-config.ts#L14).

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