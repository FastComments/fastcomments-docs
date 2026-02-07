---
Para añadir comentarios a tu sitio web creado con Vue, puedes encontrar nuestra biblioteca Vue en NPM <a href="https://www.npmjs.com/package/fastcomments-vue" target="_blank">aquí</a>.

Además, una biblioteca vue-next está en NPM <a href="https://www.npmjs.com/package/fastcomments-vue-next" target="_blank">aquí</a>

El código fuente puede encontrarse en <a href="https://github.com/FastComments" target="_blank">GitHub</a>.

El widget de comentarios FastComments para Vue soporta todas las mismas características que el de VanillaJS - live commenting, sso, y demás.

Las siguientes instrucciones son para Vue 3 ya que lleva tiempo disponible, sin embargo FastComments también soporta Vue 2 vía la biblioteca `fastcomments-vue`.

[inline-code-attrs-start title = 'FastComments Vue vía NPM'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install --save fastcomments-vue-next
[inline-code-end]


[inline-code-attrs-start title = 'FastComments Vue vía Yarn'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
yarn add fastcomments-vue-next
[inline-code-end]


[inline-code-attrs-start title = 'Ejemplo de Vue'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

Si estás en la UE, querrás establecer la `region` a `EU`:

[inline-code-attrs-start title = 'FastComments Vue - UE'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<fast-comments v-bind:config="{tenantId: 'demo', region: 'eu'}"/>
[inline-code-end]

The `fastcomments-vue` and `fastcomments-vue-next` libraries support the same configuration as the VanillaJS commenting widget.

Puedes encontrar la configuración que soporta el componente Vue <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">aquí</a>.

---