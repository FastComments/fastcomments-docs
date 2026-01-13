Puoi trovare la nostra libreria Vue su NPM <a href="https://www.npmjs.com/package/fastcomments-vue" target="_blank">qui</a>.

Inoltre, una libreria vue-next è disponibile su NPM <a href="https://www.npmjs.com/package/fastcomments-vue-next" target="_blank">qui</a>

Il codice sorgente può essere trovato su <a href="https://github.com/FastComments" target="_blank">GitHub</a>.

Il widget dei commenti FastComments per Vue supporta tutte le stesse funzionalità della versione VanillaJS - commenti in tempo reale, SSO e altro.

Le istruzioni seguenti sono per Vue 3 poiché è disponibile da tempo, tuttavia FastComments supporta anche Vue 2 tramite la libreria `fastcomments-vue`.

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

Se sei nell'UE, vorrai impostare la `region` su `EU`:

[inline-code-attrs-start title = 'FastComments Vue - EU'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<fast-comments v-bind:config="{tenantId: 'demo', region: 'eu'}"/>
[inline-code-end]

Le librerie `fastcomments-vue` e `fastcomments-vue-next` supportano la stessa configurazione del widget dei commenti VanillaJS.

Puoi trovare la configurazione supportata dal componente Vue <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">qui</a>.
