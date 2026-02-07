To add comments to your website built with Vue, you can find our Vue library on NPM <a href="https://www.npmjs.com/package/fastcomments-vue" target="_blank">here</a>.

Additionally, a vue-next library is on NPM <a href="https://www.npmjs.com/package/fastcomments-vue-next" target="_blank">here</a>

The source code can be found on <a href="https://github.com/FastComments" target="_blank">GitHub</a>.

The FastComments Vue commenting widget supports all of the same features of the VanillaJS one - live commenting, sso, and so on.

The below instructions are for Vue 3 since it has been out for some time, however FastComments also supports Vue 2 via the `fastcomments-vue` library.

[inline-code-attrs-start title = 'FastComments Vue via NPM'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install --save fastcomments-vue-next
[inline-code-end]


[inline-code-attrs-start title = 'FastComments Vue via Yarn'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
yarn add fastcomments-vue-next
[inline-code-end]


[inline-code-attrs-start title = 'Vue Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<template>
  <img alt="Vue logo" src="./assets/logo.png">
    <fast-comments v-bind:config="{tenantId: 'demo'}"/>
</template>

<script>
import {FastComments} from 'fastcomments-vue-next';

export default {
  name: 'App',
  components: {
    FastComments
  }
}
</script>
[inline-code-end]

If you're in the EU, you'll want to set the `region` to `EU`:

[inline-code-attrs-start title = 'FastComments Vue - EU'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<fast-comments v-bind:config="{tenantId: 'demo', region: 'eu'}"/>
[inline-code-end]

The `fastcomments-vue` and `fastcomments-vue-next` libraries support the same configuration as the VanillaJS commenting widget.

You can find the configuration the Vue component supports <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">here</a>.
