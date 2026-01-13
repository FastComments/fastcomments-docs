### Pripomoček za komentarje

Komponenta FastCommentsVueNext vsebuje v živo FastComments pripomoček za komentarje.

Zamenjajte "demo" spodaj z vašim "tenantId" - na voljo [tukaj](https://fastcomments.com/auth/my-account/api) v skrbniškem območju FastComments.

Pripomoček podpira veliko možnosti - poglejte FastCommentsConfig [tukaj](https://github.com/FastComments/fastcomments-typescript/blob/eae973fb7885de4df58b21b7a22a3e40c89feefa/src/fastcomments-config.ts#L14).

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