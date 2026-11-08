### कमेंट विजेट

FastCommentsVueNext कंपोनेंट में लाइव FastComments कमेंट विजेट शामिल है।

नीचे "demo" की जगह अपना "tenantId" डालें - जो FastComments एडमिन एरिया में [यहाँ](https://fastcomments.com/auth/my-account/api) उपलब्ध है।

विजेट बहुत सारे विकल्प सपोर्ट करता है - FastCommentsConfig [यहाँ](https://github.com/FastComments/fastcomments-typescript/blob/eae973fb7885de4df58b21b7a22a3e40c89feefa/src/fastcomments-config.ts#L14) देखें।

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