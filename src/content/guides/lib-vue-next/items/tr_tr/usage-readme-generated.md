### Yorum Bileşeni

FastCommentsVueNext bileşeni canlı FastComments yorum bileşenini içerir.

Aşağıdaki "demo" değerini "tenantId"niz ile değiştirin - FastComments yönetici alanında [burada](https://fastcomments.com/auth/my-account/api) bulunmaktadır.

Widget birçok seçeneği destekler - FastCommentsConfig'e [buradan](https://github.com/FastComments/fastcomments-typescript/blob/eae973fb7885de4df58b21b7a22a3e40c89feefa/src/fastcomments-config.ts#L14) bakın.

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