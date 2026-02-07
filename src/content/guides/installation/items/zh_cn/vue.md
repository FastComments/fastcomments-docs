要在使用 Vue 构建的网站上添加评论，您可以在 NPM 上找到我们的 Vue 库 <a href="https://www.npmjs.com/package/fastcomments-vue" target="_blank">此处</a>。

此外，vue-next 库也在 NPM 上，见 <a href="https://www.npmjs.com/package/fastcomments-vue-next" target="_blank">此处</a>

源代码可在 <a href="https://github.com/FastComments" target="_blank">GitHub</a> 上找到。

FastComments Vue 评论组件支持与 VanillaJS 相同的所有功能 - live commenting、sso 等。

下面的说明针对 Vue 3，因为它已经发布有一段时间；但是 FastComments 也通过 `fastcomments-vue` 库支持 Vue 2。

[inline-code-attrs-start title = '通过 NPM 安装 FastComments Vue'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install --save fastcomments-vue-next
[inline-code-end]


[inline-code-attrs-start title = '通过 Yarn 安装 FastComments Vue'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
yarn add fastcomments-vue-next
[inline-code-end]


[inline-code-attrs-start title = 'Vue 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

如果您在欧盟，应将 `region` 设置为 `EU`：

[inline-code-attrs-start title = 'FastComments Vue - 欧盟'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<fast-comments v-bind:config="{tenantId: 'demo', region: 'eu'}"/>
[inline-code-end]

`fastcomments-vue` 和 `fastcomments-vue-next` 库支持与 VanillaJS 评论组件相同的配置。

您可以在 <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">此处</a> 找到 Vue 组件支持的配置。

---