### Виджет комментариев

Компонент FastCommentsVue содержит рабочий виджет комментариев FastComments.

Замените "demo" ниже на ваш "tenantId" - доступный [здесь](https://fastcomments.com/auth/my-account/api) в панели администратора FastComments.

Виджет поддерживает множество опций - смотрите FastCommentsConfig [здесь](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts).

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