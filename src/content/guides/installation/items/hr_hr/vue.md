Našu Vue biblioteku možete pronaći na NPM-u <a href="https://www.npmjs.com/package/fastcomments-vue" target="_blank">ovdje</a>.

Dodatno, vue-next biblioteka je na NPM-u <a href="https://www.npmjs.com/package/fastcomments-vue-next" target="_blank">ovdje</a>

Izvorni kod možete pronaći na <a href="https://github.com/FastComments" target="_blank">GitHubu</a>.

FastComments Vue widget za komentare podržava sve iste značajke kao VanillaJS verzija — komentiranje u stvarnom vremenu, SSO i tako dalje.

Upute ispod su za Vue 3 budući da je izašao prije nekog vremena, međutim FastComments također podržava Vue 2 putem `fastcomments-vue` biblioteke.

[inline-code-attrs-start title = 'FastComments Vue putem NPM-a'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install --save fastcomments-vue-next
[inline-code-end]


[inline-code-attrs-start title = 'FastComments Vue putem Yarna'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
yarn add fastcomments-vue-next
[inline-code-end]


[inline-code-attrs-start title = 'Vue primjer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

Ako ste u EU, željet ćete postaviti `region` na `EU`:

[inline-code-attrs-start title = 'FastComments Vue - EU'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<fast-comments v-bind:config="{tenantId: 'demo', region: 'eu'}"/>
[inline-code-end]

Biblioteke `fastcomments-vue` i `fastcomments-vue-next` podržavaju istu konfiguraciju kao VanillaJS widget za komentare.

Konfiguraciju koju Vue komponenta podržava možete pronaći <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">ovdje</a>.
