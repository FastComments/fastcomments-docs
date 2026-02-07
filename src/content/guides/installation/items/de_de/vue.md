Um Kommentare zu Ihrer mit Vue erstellten Website hinzuzufügen, finden Sie unsere Vue-Bibliothek auf NPM <a href="https://www.npmjs.com/package/fastcomments-vue" target="_blank">hier</a>.

Außerdem ist eine vue-next-Bibliothek auf NPM <a href="https://www.npmjs.com/package/fastcomments-vue-next" target="_blank">hier</a>

Der Quellcode ist auf <a href="https://github.com/FastComments" target="_blank">GitHub</a> zu finden.

Das FastComments Vue-Kommentar-Widget unterstützt alle dieselben Funktionen wie das VanillaJS-Widget - live commenting, sso, und so weiter.

Die untenstehenden Anweisungen gelten für Vue 3, da es bereits seit einiger Zeit verfügbar ist, jedoch unterstützt FastComments auch Vue 2 über die `fastcomments-vue`-Bibliothek.

[inline-code-attrs-start title = 'FastComments Vue über NPM'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install --save fastcomments-vue-next
[inline-code-end]


[inline-code-attrs-start title = 'FastComments Vue über Yarn'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
yarn add fastcomments-vue-next
[inline-code-end]


[inline-code-attrs-start title = 'Vue-Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

Wenn Sie sich in der EU befinden, sollten Sie die `region` auf `EU` setzen:

[inline-code-attrs-start title = 'FastComments Vue - EU'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<fast-comments v-bind:config="{tenantId: 'demo', region: 'eu'}"/>
[inline-code-end]

Die Bibliotheken `fastcomments-vue` und `fastcomments-vue-next` unterstützen dieselbe Konfiguration wie das VanillaJS-Kommentar-Widget.

Die Konfiguration, die die Vue-Komponente unterstützt, finden Sie <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">hier</a>.