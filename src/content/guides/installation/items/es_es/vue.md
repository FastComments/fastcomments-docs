Puede encontrar nuestra biblioteca Vue en NPM <a href="https://www.npmjs.com/package/fastcomments-vue" target="_blank">aquí</a>.

Además, una biblioteca vue-next está disponible en NPM <a href="https://www.npmjs.com/package/fastcomments-vue-next" target="_blank">aquí</a>

El código fuente se puede encontrar en <a href="https://github.com/FastComments" target="_blank">GitHub</a>.

El widget de comentarios FastComments para Vue admite todas las mismas funciones que la versión VanillaJS: comentarios en vivo, SSO y más.

Las instrucciones a continuación son para Vue 3 ya que ha estado disponible durante algún tiempo, sin embargo, FastComments también admite Vue 2 a través de la biblioteca `fastcomments-vue`.

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

Si está en la UE, querrá establecer la `region` a `EU`:

[inline-code-attrs-start title = 'FastComments Vue - EU'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<fast-comments v-bind:config="{tenantId: 'demo', region: 'eu'}"/>
[inline-code-end]

Las bibliotecas `fastcomments-vue` y `fastcomments-vue-next` admiten la misma configuración que el widget de comentarios VanillaJS.

Puede encontrar la configuración que admite el componente Vue <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">aquí</a>.
