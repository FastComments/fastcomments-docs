Da biste dodali komentare na vašu web stranicu izgrađenu s Vue, naš Vue paket možete pronaći na NPM-u <a href="https://www.npmjs.com/package/fastcomments-vue" target="_blank">ovdje</a>.

Također, biblioteku vue-next možete pronaći na NPM-u <a href="https://www.npmjs.com/package/fastcomments-vue-next" target="_blank">ovdje</a>

Izvorni kod možete pronaći na <a href="https://github.com/FastComments" target="_blank">GitHubu</a>.

FastComments Vue widget za komentare podržava sve iste značajke kao i VanillaJS - komentiranje uživo, sso i slično.

Upute u nastavku odnose se na Vue 3 jer je već neko vrijeme dostupan, no FastComments također podržava Vue 2 putem biblioteke `fastcomments-vue`.

[inline-code-attrs-start title = 'FastComments Vue preko NPM'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install --save fastcomments-vue-next
[inline-code-end]


[inline-code-attrs-start title = 'FastComments Vue preko Yarn'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
yarn add fastcomments-vue-next
[inline-code-end]


[inline-code-attrs-start title = 'Primjer Vue'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

Ako se nalazite u EU, trebali biste postaviti `region` na `EU`:

[inline-code-attrs-start title = 'FastComments Vue - EU'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<fast-comments v-bind:config="{tenantId: 'demo', region: 'eu'}"/>
[inline-code-end]

Biblioteke `fastcomments-vue` i `fastcomments-vue-next` podržavaju istu konfiguraciju kao i VanillaJS widget za komentare.

Konfiguraciju koju Vue komponenta podržava možete pronaći <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">ovdje</a>.