您可以在NPM的<a href="https://www.npmjs.com/package/fastcomments-vue" target="_blank">这里</a>找到我们的Vue库。

此外，vue-next库在NPM的<a href="https://www.npmjs.com/package/fastcomments-vue-next" target="_blank">这里</a>

源代码可以在<a href="https://github.com/FastComments" target="_blank">GitHub</a>上找到。

FastComments Vue评论小部件支持与VanillaJS版本相同的所有功能——实时评论、SSO等。

以下说明适用于Vue 3（因为它已经发布了一段时间），但FastComments也通过`fastcomments-vue`库支持Vue 2。

[inline-code-attrs-start title = 'FastComments Vue（通过NPM）'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install --save fastcomments-vue-next
[inline-code-end]


[inline-code-attrs-start title = 'FastComments Vue（通过Yarn）'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
yarn add fastcomments-vue-next
[inline-code-end]


[inline-code-attrs-start title = 'Vue示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

如果您在欧盟，您需要将`region`设置为`EU`：

[inline-code-attrs-start title = 'FastComments Vue - 欧盟'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<fast-comments v-bind:config="{tenantId: 'demo', region: 'eu'}"/>
[inline-code-end]

`fastcomments-vue`和`fastcomments-vue-next`库支持与VanillaJS评论小部件相同的配置。

您可以在<a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">这里</a>找到Vue组件支持的配置。
