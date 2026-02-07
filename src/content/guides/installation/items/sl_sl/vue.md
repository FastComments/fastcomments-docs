Če želite dodati komentarje na svoje spletno mesto, zgrajeno z Vue, lahko našo knjižnico za Vue najdete na NPM <a href="https://www.npmjs.com/package/fastcomments-vue" target="_blank">tukaj</a>.

Poleg tega je knjižnica vue-next na NPM <a href="https://www.npmjs.com/package/fastcomments-vue-next" target="_blank">tukaj</a>

Izvorno kodo najdete na <a href="https://github.com/FastComments" target="_blank">GitHub</a>.

FastComments Vue komentarni pripomoček podpira vse iste funkcije kot VanillaJS - komentiranje v živo, sso in podobno.

Spodnja navodila so za Vue 3, saj je na voljo že nekaj časa, vendar FastComments podpira tudi Vue 2 prek knjižnice `fastcomments-vue`.

[inline-code-attrs-start title = 'FastComments Vue prek NPM'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install --save fastcomments-vue-next
[inline-code-end]


[inline-code-attrs-start title = 'FastComments Vue prek Yarn'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
yarn add fastcomments-vue-next
[inline-code-end]


[inline-code-attrs-start title = 'Primer za Vue'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

Če ste v EU, nastavite `region` na `EU`:

[inline-code-attrs-start title = 'FastComments Vue - EU'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<fast-comments v-bind:config="{tenantId: 'demo', region: 'eu'}"/>
[inline-code-end]

Knjižnici `fastcomments-vue` in `fastcomments-vue-next` podpirata enako konfiguracijo kot VanillaJS komentarni pripomoček.

Konfiguracijo, ki jo komponenta Vue podpira, najdete <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">tukaj</a>.