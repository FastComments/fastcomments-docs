---
### Виџет за коментаре

Компонента FastCommentsVue садржи виџет за коментаре FastComments који ради у реалном времену.

Замените "demo" испод са вашим "tenantId" - доступан [овдје](https://fastcomments.com/auth/my-account/api) у FastComments административном подручју.

Виџет подржава много опција - погледајте FastCommentsConfig [овдје](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts).

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