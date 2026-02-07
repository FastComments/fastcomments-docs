Pour ajouter des commentaires à votre site web construit avec Vue, vous pouvez trouver notre bibliothèque Vue sur NPM <a href="https://www.npmjs.com/package/fastcomments-vue" target="_blank">ici</a>.

De plus, une bibliothèque vue-next est disponible sur NPM <a href="https://www.npmjs.com/package/fastcomments-vue-next" target="_blank">ici</a>

Le code source est disponible sur <a href="https://github.com/FastComments" target="_blank">GitHub</a>.

Le widget de commentaires FastComments pour Vue prend en charge toutes les mêmes fonctionnalités que celui en VanillaJS — commentaires en direct, SSO, etc.

Les instructions ci‑dessous concernent Vue 3 car il est disponible depuis un certain temps ; cependant, FastComments prend également en charge Vue 2 via la bibliothèque `fastcomments-vue`.

[inline-code-attrs-start title = 'FastComments Vue via NPM'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install --save fastcomments-vue-next
[inline-code-end]


[inline-code-attrs-start title = 'FastComments Vue via Yarn'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
yarn add fastcomments-vue-next
[inline-code-end]


[inline-code-attrs-start title = 'Exemple Vue'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

Si vous êtes dans l'UE, vous devrez définir `region` sur `EU`:

[inline-code-attrs-start title = 'FastComments Vue - UE'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<fast-comments v-bind:config="{tenantId: 'demo', region: 'eu'}"/>
[inline-code-end]

The `fastcomments-vue` and `fastcomments-vue-next` libraries support the same configuration as the VanillaJS commenting widget.

You can find the configuration the Vue component supports <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">here</a>.