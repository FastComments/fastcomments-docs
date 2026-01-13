### Віджет коментарів

Компонент FastCommentsVue містить живий віджет коментарів FastComments.

Замініть "demo" нижче на свій "tenantId" — доступний [тут](https://fastcomments.com/auth/my-account/api) в адмін-панелі FastComments.

Віджет підтримує багато параметрів — див. FastCommentsConfig [тут](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts).

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