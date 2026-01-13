### Коментарният уиджет

Компонентът FastCommentsVue съдържа уиджета за коментари на FastComments в реално време.

Заменете "demo" по-долу с вашия "tenantId" - наличен [тук](https://fastcomments.com/auth/my-account/api) в административния панел на FastComments.

Уиджетът поддържа множество опции - вижте FastCommentsConfig [тук](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts).

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