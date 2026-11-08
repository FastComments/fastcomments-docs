### कमेंट विजेट

FastCommentsVue कंपोनेंट में लाइव FastComments कमेंट विजेट शामिल है।

नीचे "demo" को अपने "tenantId" से बदलें - जो FastComments एडमिन एरिया में [यहाँ](https://fastcomments.com/auth/my-account/api) उपलब्ध है।

विजेट बहुत सारे विकल्पों का समर्थन करता है - FastCommentsConfig [यहाँ](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts) देखें।

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