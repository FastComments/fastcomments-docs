Da biste dodali komentare na svoju web stranicu izgrađenu koristeći Vue, naš Vue paket možete pronaći na NPM-u <a href="https://www.npmjs.com/package/fastcomments-vue" target="_blank">ovdje</a>.

Takođe, biblioteka vue-next je na NPM-u <a href="https://www.npmjs.com/package/fastcomments-vue-next" target="_blank">ovdje</a>

Izvorni kod možete pronaći na <a href="https://github.com/FastComments" target="_blank">GitHub</a>.

FastComments Vue vidžet za komentare podržava sve iste funkcije kao i VanillaJS - komentarisanje uživo, SSO i slično.

Upute ispod su za Vue 3 jer je već neko vrijeme dostupan, međutim FastComments takođe podržava Vue 2 putem biblioteke `fastcomments-vue`.

[inline-code-attrs-start title = 'FastComments Vue putem NPM-a'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install --save fastcomments-vue-next
[inline-code-end]


[inline-code-attrs-start title = 'FastComments Vue putem Yarn-a'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
yarn add fastcomments-vue-next
[inline-code-end]


[inline-code-attrs-start title = 'Primjer za Vue'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

Ako ste u EU, trebate postaviti `region` na `EU`:

[inline-code-attrs-start title = 'FastComments Vue - EU'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<fast-comments v-bind:config="{tenantId: 'demo', region: 'eu'}"/>
[inline-code-end]

Biblioteke `fastcomments-vue` i `fastcomments-vue-next` podržavaju istu konfiguraciju kao VanillaJS vidžet za komentare.

Konfiguraciju koju Vue komponenta podržava možete pronaći <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">ovdje</a>.