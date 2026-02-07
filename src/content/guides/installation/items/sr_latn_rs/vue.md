Da biste dodali komentare na vaš vebsajt napravljen u Vue, možete pronaći našu Vue biblioteku na NPM <a href="https://www.npmjs.com/package/fastcomments-vue" target="_blank">ovde</a>.

Pored toga, biblioteka vue-next je na NPM <a href="https://www.npmjs.com/package/fastcomments-vue-next" target="_blank">ovde</a>

Izvorni kod se može naći na <a href="https://github.com/FastComments" target="_blank">GitHub</a>.

FastComments Vue widget za komentare podržava sve iste funkcije kao i VanillaJS — komentarisanje uživo, sso, i tako dalje.

Sledeće instrukcije su za Vue 3 pošto je izašao već neko vreme, međutim FastComments takođe podržava Vue 2 preko biblioteke `fastcomments-vue`.

[inline-code-attrs-start title = 'FastComments Vue preko NPM'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install --save fastcomments-vue-next
[inline-code-end]


[inline-code-attrs-start title = 'FastComments Vue preko Yarn'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
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

Ako ste u EU, trebalo bi da podesite `region` na `EU`:

[inline-code-attrs-start title = 'FastComments Vue - EU'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<fast-comments v-bind:config="{tenantId: 'demo', region: 'eu'}"/>
[inline-code-end]

Biblioteke `fastcomments-vue` i `fastcomments-vue-next` podržavaju istu konfiguraciju kao i VanillaJS widget za komentare.

Konfiguraciju koju Vue komponenta podržava možete pronaći <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">ovde</a>.