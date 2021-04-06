You can find our Vue library on NPM <a href="https://www.npmjs.com/package/fastcomments-vue" target="_blank">here</a>.

Additionally, a vue-next library is on NPM <a href="https://www.npmjs.com/package/fastcomments-vue-next" target="_blank">here</a>

The source code can be found on <a href="https://github.com/FastComments" target="_blank">GitHub</a>.

The FastComments Vue commenting widget supports all of the same features of the VanillaJS one - live commenting, sso, and so on.

[inline-code-attrs-start title = 'FastComments Vue via NPM'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install --save fastcomments-vue
[inline-code-end]


[inline-code-attrs-start title = 'FastComments Vue via Yarn'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
yarn add fastcomments-vue
[inline-code-end]


[inline-code-attrs-start title = 'Vue Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
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
[inline-code-end]

The fastcomments-vue and fastcomments-vue-next libraries support the same configuration as the VanillaJS commenting widget.

You can find the configuration the Vue component supports <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">here</a>.
