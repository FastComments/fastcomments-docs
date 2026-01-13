Du kan finde vores Vue-bibliotek på NPM <a href="https://www.npmjs.com/package/fastcomments-vue" target="_blank">her</a>.

Derudover findes vue-next-biblioteket på NPM <a href="https://www.npmjs.com/package/fastcomments-vue-next" target="_blank">her</a>

Kildekoden kan findes på <a href="https://github.com/FastComments" target="_blank">GitHub</a>.

FastComments Vue kommentar-widget understøtter alle de samme funktioner som VanillaJS-versionen - live kommentering, SSO og så videre.

Instruktionerne nedenfor er for Vue 3, da det har været ude i noget tid, men FastComments understøtter også Vue 2 via `fastcomments-vue`-biblioteket.

[inline-code-attrs-start title = 'FastComments Vue via NPM'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install --save fastcomments-vue-next
[inline-code-end]


[inline-code-attrs-start title = 'FastComments Vue via Yarn'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
yarn add fastcomments-vue-next
[inline-code-end]


[inline-code-attrs-start title = 'Vue eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

Hvis du er i EU, vil du gerne sætte `region` til `EU`:

[inline-code-attrs-start title = 'FastComments Vue - EU'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<fast-comments v-bind:config="{tenantId: 'demo', region: 'eu'}"/>
[inline-code-end]

`fastcomments-vue` og `fastcomments-vue-next` bibliotekerne understøtter den samme konfiguration som VanillaJS kommentar-widget.

Du kan finde konfigurationen, som Vue-komponenten understøtter <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">her</a>.
