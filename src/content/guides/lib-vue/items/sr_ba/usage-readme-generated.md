### Видгет за коментаре

Компонента FastCommentsVue садржи видгет за коментаре FastComments који ради у реалном времену.

Замените "demo" доле са вашим "tenantId" - доступним [овдје](https://fastcomments.com/auth/my-account/api) у администраторском делу FastComments.

Видгет подржава мноштво опција - погледајте FastCommentsConfig [овдје](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts).

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