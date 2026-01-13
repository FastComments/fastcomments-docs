### コメントウィジェット

The FastCommentsVueNext componentには、ライブの FastComments コメントウィジェットが含まれます。

下の "demo" をあなたの "tenantId" に置き換えてください - FastComments 管理画面で [ここ](https://fastcomments.com/auth/my-account/api) から取得できます。

このウィジェットは多数のオプションをサポートしています - FastCommentsConfig は [ここ](https://github.com/FastComments/fastcomments-typescript/blob/eae973fb7885de4df58b21b7a22a3e40c89feefa/src/fastcomments-config.ts#L14) を参照してください。

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