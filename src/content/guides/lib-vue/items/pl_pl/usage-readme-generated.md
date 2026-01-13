### Widżet komentarzy

Komponent FastCommentsVue zawiera działający widżet komentarzy FastComments.

Zastąp poniżej "demo" swoim "tenantId" - dostępnym [tutaj](https://fastcomments.com/auth/my-account/api) w panelu administracyjnym FastComments.

Widżet obsługuje wiele opcji - zobacz FastCommentsConfig [tutaj](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts).

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