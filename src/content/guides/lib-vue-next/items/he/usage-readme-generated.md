---
### וידג'ט התגובות

הקומפוננטה FastCommentsVueNext מכילה את ווידג'ט התגובות החי של FastComments.

החליפו את "demo" למטה ב-"tenantId" שלכם - זמין [כאן](https://fastcomments.com/auth/my-account/api) באזור הניהול של FastComments.

הווידג'ט תומך בהרבה אפשרויות - ראו את FastCommentsConfig [כאן](https://github.com/FastComments/fastcomments-typescript/blob/eae973fb7885de4df58b21b7a22a3e40c89feefa/src/fastcomments-config.ts#L14).

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