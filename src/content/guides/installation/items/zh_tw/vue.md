若要在使用 Vue 建置的網站中新增留言，您可以在我們的 NPM 上找到 Vue 函式庫 <a href="https://www.npmjs.com/package/fastcomments-vue" target="_blank">這裡</a>。

此外，vue-next 函式庫也已在 NPM 上發布 <a href="https://www.npmjs.com/package/fastcomments-vue-next" target="_blank">這裡</a>

原始程式碼可以在 <a href="https://github.com/FastComments" target="_blank">GitHub</a> 上找到。

FastComments Vue 留言小工具支援與 VanillaJS 版本相同的所有功能 - 即時留言、SSO 等等。

以下說明適用於 Vue 3，因為它已推出一段時間，但 FastComments 也透過 `fastcomments-vue` 函式庫支援 Vue 2。

[inline-code-attrs-start title = '透過 NPM 安裝 FastComments Vue'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install --save fastcomments-vue-next
[inline-code-end]


[inline-code-attrs-start title = '透過 Yarn 安裝 FastComments Vue'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
yarn add fastcomments-vue-next
[inline-code-end]


[inline-code-attrs-start title = 'Vue 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

如果您位於歐盟 (EU)，您需要將 `region` 設定為 `EU`：

[inline-code-attrs-start title = 'FastComments Vue - EU'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<fast-comments v-bind:config="{tenantId: 'demo', region: 'eu'}"/>
[inline-code-end]

`fastcomments-vue` 與 `fastcomments-vue-next` 函式庫支援與 VanillaJS 留言小工具相同的設定。

您可以在 <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">這裡</a> 找到 Vue 元件支援的設定。