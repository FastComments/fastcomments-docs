### Το Widget Σχολίων

Το στοιχείο FastCommentsVue περιέχει το ζωντανό widget σχολίων του FastComments.

Αντικαταστήστε το "demo" πιο κάτω με το "tenantId" σας - διαθέσιμο [εδώ](https://fastcomments.com/auth/my-account/api) στην περιοχή διαχείρισης του FastComments.

Το widget υποστηρίζει πολλές επιλογές - δείτε το FastCommentsConfig [εδώ](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts).

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