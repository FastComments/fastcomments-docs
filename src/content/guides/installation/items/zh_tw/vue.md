您可以在NPM的<a href="https://www.npmjs.com/package/fastcomments-vue" target="_blank">這裡</a>找到我們的Vue函式庫。

此外，vue-next函式庫在NPM的<a href="https://www.npmjs.com/package/fastcomments-vue-next" target="_blank">這裡</a>

原始碼可以在<a href="https://github.com/FastComments" target="_blank">GitHub</a>上找到。

FastComments Vue評論小工具支援與VanillaJS版本相同的所有功能——即時評論、SSO等。

以下說明適用於Vue 3（因為它已經發布了一段時間），但FastComments也透過`fastcomments-vue`函式庫支援Vue 2。

[inline-code-attrs-start title = 'FastComments Vue（透過NPM）'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install --save fastcomments-vue-next
[inline-code-end]


[inline-code-attrs-start title = 'FastComments Vue（透過Yarn）'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
yarn add fastcomments-vue-next
[inline-code-end]


[inline-code-attrs-start title = 'Vue範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

如果您在歐盟，您需要將`region`設定為`EU`：

[inline-code-attrs-start title = 'FastComments Vue - 歐盟'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<fast-comments v-bind:config="{tenantId: 'demo', region: 'eu'}"/>
[inline-code-end]

`fastcomments-vue`和`fastcomments-vue-next`函式庫支援與VanillaJS評論小工具相同的配置。

您可以在<a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">這裡</a>找到Vue元件支援的配置。
