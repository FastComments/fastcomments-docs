Vue kütüphanemizi NPM'de <a href="https://www.npmjs.com/package/fastcomments-vue" target="_blank">burada</a> bulabilirsiniz.

Ayrıca, vue-next kütüphanesi NPM'de <a href="https://www.npmjs.com/package/fastcomments-vue-next" target="_blank">burada</a> mevcuttur

Kaynak kodu <a href="https://github.com/FastComments" target="_blank">GitHub</a>'da bulunabilir.

FastComments Vue yorum widget'ı, VanillaJS olanla aynı tüm özellikleri destekler - canlı yorumlama, SSO vb.

Aşağıdaki talimatlar Vue 3 içindir (bir süredir piyasada olduğundan), ancak FastComments `fastcomments-vue` kütüphanesi aracılığıyla Vue 2'yi de destekler.

[inline-code-attrs-start title = 'FastComments Vue (NPM ile)'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install --save fastcomments-vue-next
[inline-code-end]


[inline-code-attrs-start title = 'FastComments Vue (Yarn ile)'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
yarn add fastcomments-vue-next
[inline-code-end]


[inline-code-attrs-start title = 'Vue Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

AB'deyseniz, `region`'ı `EU` olarak ayarlamanız gerekir:

[inline-code-attrs-start title = 'FastComments Vue - AB'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<fast-comments v-bind:config="{tenantId: 'demo', region: 'eu'}"/>
[inline-code-end]

`fastcomments-vue` ve `fastcomments-vue-next` kütüphaneleri, VanillaJS yorum widget'ı ile aynı yapılandırmayı destekler.

Vue bileşeninin desteklediği yapılandırmayı <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">burada</a> bulabilirsiniz.
