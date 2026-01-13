### コメントウィジェット

FastCommentsVue コンポーネントには、ライブの FastComments コメントウィジェットが含まれています。

下の "demo" をあなたの "tenantId" に置き換えてください - tenantId は FastComments 管理画面の [ここ](https://fastcomments.com/auth/my-account/api) で確認できます。

ウィジェットは多数のオプションをサポートしています - 詳細は FastCommentsConfig を[こちら](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts)でご確認ください。

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