---
### 评论小部件

FastCommentsVue 组件包含实时 FastComments 评论小部件。

将下面的 "demo" 替换为您的 "tenantId" — 可在 FastComments 管理区域的 [此处](https://fastcomments.com/auth/my-account/api) 获取。

该小部件支持许多选项 - 请参阅 FastCommentsConfig 的 [此处](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts)。

```vue
<script lang="ts">
import Vue from 'vue';
import FastCommentsVue from 'fastcomments-vue';

export default Vue.extend({
  name: 'ServeDev',
  components: {
    FastCommentsVue
  }
});
</script>

<template>
  <div id="app">
    <fast-comments-vue v-bind:config="{tenantId: 'demo'}" />
  </div>
</template>
```
---