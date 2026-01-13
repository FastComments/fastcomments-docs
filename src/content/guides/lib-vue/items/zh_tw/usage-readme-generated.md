### 評論小工具

FastCommentsVue component 包含即時的 FastComments 評論小工具。

請將下方的 "demo" 替換為您的 "tenantId" - 可在 FastComments 管理區的 [這裡](https://fastcomments.com/auth/my-account/api) 取得。

此小工具支援許多選項 - 請在 [這裡](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts) 查看 FastCommentsConfig。

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