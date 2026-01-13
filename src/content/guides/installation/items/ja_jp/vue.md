VueライブラリはNPMの<a href="https://www.npmjs.com/package/fastcomments-vue" target="_blank">こちら</a>で見つけることができます。

また、vue-nextライブラリはNPMの<a href="https://www.npmjs.com/package/fastcomments-vue-next" target="_blank">こちら</a>にあります

ソースコードは<a href="https://github.com/FastComments" target="_blank">GitHub</a>で見つけることができます。

FastComments Vueコメントウィジェットは、VanillaJS版と同じすべての機能（ライブコメント、SSO など）をサポートしています。

以下の手順はVue 3向けです（リリースから時間が経っているため）。ただし、FastCommentsは`fastcomments-vue`ライブラリを通じてVue 2もサポートしています。

[inline-code-attrs-start title = 'FastComments Vue（NPM経由）'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install --save fastcomments-vue-next
[inline-code-end]


[inline-code-attrs-start title = 'FastComments Vue（Yarn経由）'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
yarn add fastcomments-vue-next
[inline-code-end]


[inline-code-attrs-start title = 'Vueの例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

EUにいる場合は、`region`を`EU`に設定する必要があります：

[inline-code-attrs-start title = 'FastComments Vue - EU'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<fast-comments v-bind:config="{tenantId: 'demo', region: 'eu'}"/>
[inline-code-end]

`fastcomments-vue`と`fastcomments-vue-next`ライブラリは、VanillaJSコメントウィジェットと同じ設定をサポートしています。

Vueコンポーネントがサポートする設定は<a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">こちら</a>で見つけることができます。
