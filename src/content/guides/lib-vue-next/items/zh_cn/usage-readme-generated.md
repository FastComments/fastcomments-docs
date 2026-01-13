### 评论小部件

FastCommentsVueNext 组件包含实时的 FastComments 评论小部件。

将下面的 "demo" 替换为你的 "tenantId" — 在 FastComments 管理界面的[此处](https://fastcomments.com/auth/my-account/api) 可用。

该小部件支持许多选项 — 有关详细信息，请参见 FastCommentsConfig 的[此处](https://github.com/FastComments/fastcomments-typescript/blob/eae973fb7885de4df58b21b7a22a3e40c89feefa/src/fastcomments-config.ts#L14)。

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