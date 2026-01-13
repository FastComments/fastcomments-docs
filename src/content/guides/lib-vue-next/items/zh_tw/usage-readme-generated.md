### 評論小工具

FastCommentsVueNext 元件包含即時的 FastComments 評論小工具。

將下方的 "demo" 替換為您的 "tenantId" - 可在 FastComments 管理介面的 [這裡](https://fastcomments.com/auth/my-account/api) 取得。

此小工具支援許多選項 - 請參閱 FastCommentsConfig [這裡](https://github.com/FastComments/fastcomments-typescript/blob/eae973fb7885de4df58b21b7a22a3e40c89feefa/src/fastcomments-config.ts#L14)。

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