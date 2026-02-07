若要在以 Vue 建置的網站中加入留言，您可以在 NPM 找到我們的 Vue 函式庫，請見 <a href="https://www.npmjs.com/package/fastcomments-vue" target="_blank">這裡</a>。

此外，vue-next 函式庫也可在 NPM 找到，請見 <a href="https://www.npmjs.com/package/fastcomments-vue-next" target="_blank">這裡</a>

原始程式碼可在 <a href="https://github.com/FastComments" target="_blank">GitHub</a> 找到。

FastComments Vue 留言元件支援與 VanillaJS 相同的所有功能 - 即時留言、sso，等等。

以下說明針對 Vue 3，因為它已經推出一段時間；不過 FastComments 也透過 `fastcomments-vue` 函式庫支援 Vue 2。

[inline-code-attrs-start title = '透過 NPM 的 FastComments Vue'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install --save fastcomments-vue-next
[inline-code-end]


[inline-code-attrs-start title = '透過 Yarn 的 FastComments Vue'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
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

如果您位於歐盟，您會想將 `region` 設為 `EU`：

[inline-code-attrs-start title = 'FastComments Vue - 歐盟'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<fast-comments v-bind:config="{tenantId: 'demo', region: 'eu'}"/>
[inline-code-end]

The `fastcomments-vue` and `fastcomments-vue-next` libraries support the same configuration as the VanillaJS commenting widget.

您可以在 <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">這裡</a> 找到 Vue 元件支援的設定。

---