### ווידג'ט התגובות

קומפוננטת FastCommentsVue מכילה את ווידג'ט התגובות החי של FastComments.

החליפו את "demo" למטה ב-"tenantId" שלכם - זמין [כאן](https://fastcomments.com/auth/my-account/api) באזור הניהול של FastComments.

הווידג'ט תומך בהרבה אפשרויות - ראו את FastCommentsConfig [כאן](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts).

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