Vueで構築されたウェブサイトにコメントを追加するには、当社のVueライブラリがNPMで入手できます <a href="https://www.npmjs.com/package/fastcomments-vue" target="_blank">こちら</a>。

さらに、vue-next用のライブラリもNPMにあります <a href="https://www.npmjs.com/package/fastcomments-vue-next" target="_blank">こちら</a>

ソースコードは <a href="https://github.com/FastComments" target="_blank">GitHub</a> で見つけることができます。

FastComments Vue のコメントウィジェットは、VanillaJS のものと同じすべての機能（ライブコメント、sso、など）をサポートします。

以下の手順は Vue 3 向けのものです（しばらく前から利用可能なため）。ただし、FastComments は `fastcomments-vue` ライブラリを通じて Vue 2 もサポートしています。

[inline-code-attrs-start title = 'NPM 経由の FastComments Vue'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install --save fastcomments-vue-next
[inline-code-end]


[inline-code-attrs-start title = 'Yarn 経由の FastComments Vue'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
yarn add fastcomments-vue-next
[inline-code-end]


[inline-code-attrs-start title = 'Vue の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

EU にいる場合は、`region` を `EU` に設定してください：

[inline-code-attrs-start title = 'EU向けの FastComments Vue'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<fast-comments v-bind:config="{tenantId: 'demo', region: 'eu'}"/>
[inline-code-end]

`fastcomments-vue` と `fastcomments-vue-next` ライブラリは、VanillaJS コメントウィジェットと同じ設定をサポートします。

Vue コンポーネントがサポートする設定は <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">こちら</a> で確認できます。

---