### Віджет коментарів

Компонент FastCommentsVueNext містить живий віджет коментарів FastComments.

Замініть "demo" нижче на ваш "tenantId" — доступний [тут](https://fastcomments.com/auth/my-account/api) у панелі адміністратора FastComments.

Віджет підтримує багато параметрів — див. FastCommentsConfig [тут](https://github.com/FastComments/fastcomments-typescript/blob/eae973fb7885de4df58b21b7a22a3e40c89feefa/src/fastcomments-config.ts#L14).

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