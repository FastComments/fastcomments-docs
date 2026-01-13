U kunt onze Vue-bibliotheek op NPM <a href="https://www.npmjs.com/package/fastcomments-vue" target="_blank">hier</a> vinden.

Daarnaast is een vue-next-bibliotheek beschikbaar op NPM <a href="https://www.npmjs.com/package/fastcomments-vue-next" target="_blank">hier</a>

De broncode is te vinden op <a href="https://github.com/FastComments" target="_blank">GitHub</a>.

De FastComments Vue reactiewidget ondersteunt alle dezelfde functies als de VanillaJS-versie - live reacties, SSO, enzovoort.

De onderstaande instructies zijn voor Vue 3 aangezien het al enige tijd beschikbaar is, maar FastComments ondersteunt ook Vue 2 via de `fastcomments-vue`-bibliotheek.

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

Als u in de EU bent, wilt u de `region` instellen op `EU`:

[inline-code-attrs-start title = 'FastComments Vue - EU'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<fast-comments v-bind:config="{tenantId: 'demo', region: 'eu'}"/>
[inline-code-end]

De `fastcomments-vue` en `fastcomments-vue-next` bibliotheken ondersteunen dezelfde configuratie als de VanillaJS reactiewidget.

U kunt de configuratie die de Vue-component ondersteunt <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">hier</a> vinden.
