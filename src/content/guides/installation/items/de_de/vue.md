Sie finden unsere Vue-Bibliothek auf NPM <a href="https://www.npmjs.com/package/fastcomments-vue" target="_blank">hier</a>.

Zusätzlich ist eine vue-next-Bibliothek auf NPM <a href="https://www.npmjs.com/package/fastcomments-vue-next" target="_blank">hier</a> verfügbar.

Den Quellcode finden Sie auf <a href="https://github.com/FastComments" target="_blank">GitHub</a>.

Das FastComments Vue-Kommentar-Widget unterstützt alle Funktionen der VanillaJS-Version - Live-Kommentare, SSO und mehr.

Die folgenden Anweisungen gelten für Vue 3, da es schon eine Weile verfügbar ist. FastComments unterstützt jedoch auch Vue 2 über die `fastcomments-vue`-Bibliothek.

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

Wenn Sie in der EU sind, sollten Sie die `region` auf `EU` setzen:

[inline-code-attrs-start title = 'FastComments Vue - EU'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<fast-comments v-bind:config="{tenantId: 'demo', region: 'eu'}"/>
[inline-code-end]

Die `fastcomments-vue`- und `fastcomments-vue-next`-Bibliotheken unterstützen dieselbe Konfiguration wie das VanillaJS-Kommentar-Widget.

Die vom Vue-Komponenten unterstützte Konfiguration finden Sie <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">hier</a>.
