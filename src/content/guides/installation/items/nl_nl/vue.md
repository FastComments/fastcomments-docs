---
Om opmerkingen toe te voegen aan je website die gebouwd is met Vue, kun je onze Vue-bibliotheek op NPM <a href="https://www.npmjs.com/package/fastcomments-vue" target="_blank">hier</a> vinden.

Daarnaast is er een vue-next-bibliotheek op NPM <a href="https://www.npmjs.com/package/fastcomments-vue-next" target="_blank">hier</a>

De broncode is te vinden op <a href="https://github.com/FastComments" target="_blank">GitHub</a>.

De FastComments Vue-commentaarwidget ondersteunt alle dezelfde functies als die van VanillaJS - live commentaar, sso, enzovoort.

De onderstaande instructies zijn voor Vue 3 omdat het al een tijdje uit is, maar FastComments ondersteunt ook Vue 2 via de `fastcomments-vue`-bibliotheek.

[inline-code-attrs-start title = 'FastComments Vue via NPM'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install --save fastcomments-vue-next
[inline-code-end]


[inline-code-attrs-start title = 'FastComments Vue via Yarn'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
yarn add fastcomments-vue-next
[inline-code-end]


[inline-code-attrs-start title = 'Vue Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

Als je in de EU bent, wil je de `region` op `EU` zetten:

[inline-code-attrs-start title = 'FastComments Vue - EU'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<fast-comments v-bind:config="{tenantId: 'demo', region: 'eu'}"/>
[inline-code-end]

De `fastcomments-vue` en `fastcomments-vue-next` bibliotheken ondersteunen dezelfde configuratie als de VanillaJS-commentaarwidget.

Je kunt de configuratie die de Vue-component ondersteunt <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">hier</a> vinden.

---