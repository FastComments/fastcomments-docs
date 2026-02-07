Vue ile oluşturulmuş web sitenize yorum eklemek için Vue kütüphanemizi NPM'de <a href="https://www.npmjs.com/package/fastcomments-vue" target="_blank">burada</a> bulabilirsiniz.

Ayrıca, vue-next kütüphanesi NPM'de <a href="https://www.npmjs.com/package/fastcomments-vue-next" target="_blank">burada</a>

Kaynak kodu <a href="https://github.com/FastComments" target="_blank">GitHub</a>'da bulunabilir.

FastComments Vue yorum bileşeni VanillaJS olanla aynı tüm özellikleri destekler - canlı yorum, sso ve benzeri.

Aşağıdaki talimatlar Vue 3 içindir çünkü bir süredir kullanımdadır, ancak FastComments ayrıca Vue 2'yi `fastcomments-vue` kütüphanesi aracılığıyla da destekler.

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

Eğer AB içindeyseniz, `region` değerini `EU` olarak ayarlamak isteyebilirsiniz:

[inline-code-attrs-start title = 'FastComments Vue - EU'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<fast-comments v-bind:config="{tenantId: 'demo', region: 'eu'}"/>
[inline-code-end]

`fastcomments-vue` ve `fastcomments-vue-next` kütüphaneleri, VanillaJS yorum bileşeniyle aynı yapılandırmayı destekler.

Vue bileşeninin desteklediği yapılandırmayı <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">burada</a> bulabilirsiniz.

---