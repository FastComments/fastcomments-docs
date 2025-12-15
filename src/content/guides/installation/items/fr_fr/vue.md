Vous pouvez trouver notre bibliothèque Vue sur NPM <a href="https://www.npmjs.com/package/fastcomments-vue" target="_blank">ici</a>.

De plus, une bibliothèque vue-next est disponible sur NPM <a href="https://www.npmjs.com/package/fastcomments-vue-next" target="_blank">ici</a>

Le code source se trouve sur <a href="https://github.com/FastComments" target="_blank">GitHub</a>.

Le widget de commentaires FastComments pour Vue prend en charge toutes les mêmes fonctionnalités que la version VanillaJS - commentaires en direct, SSO, et plus encore.

Les instructions ci-dessous sont pour Vue 3 puisqu'il est sorti depuis un certain temps, cependant FastComments prend également en charge Vue 2 via la bibliothèque `fastcomments-vue`.

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

Si vous êtes dans l'UE, vous devrez définir la `region` sur `EU` :

[inline-code-attrs-start title = 'FastComments Vue - EU'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<fast-comments v-bind:config="{tenantId: 'demo', region: 'eu'}"/>
[inline-code-end]

Les bibliothèques `fastcomments-vue` et `fastcomments-vue-next` prennent en charge la même configuration que le widget de commentaires VanillaJS.

Vous pouvez trouver la configuration prise en charge par le composant Vue <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">ici</a>.
