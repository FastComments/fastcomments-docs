Para añadir comentarios a tu sitio web construido con Vue, puedes encontrar nuestra librería de Vue en NPM <a href="https://www.npmjs.com/package/fastcomments-vue" target="_blank">aquí</a>.

Además, hay una librería vue-next en NPM <a href="https://www.npmjs.com/package/fastcomments-vue-next" target="_blank">aquí</a>

El código fuente se puede encontrar en <a href="https://github.com/FastComments" target="_blank">GitHub</a>.

El widget de comentarios de FastComments para Vue soporta todas las mismas características que el de VanillaJS - comentarios en vivo, sso, y más.

Las instrucciones de abajo son para Vue 3 ya que ha estado disponible desde hace tiempo, sin embargo FastComments también soporta Vue 2 a través de la librería `fastcomments-vue`.

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

Si estás en la UE, querrás establecer `region` como `EU`:

[inline-code-attrs-start title = 'FastComments Vue - UE'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<fast-comments v-bind:config="{tenantId: 'demo', region: 'eu'}"/>
[inline-code-end]

Las librerías `fastcomments-vue` y `fastcomments-vue-next` soportan la misma configuración que el widget de comentarios de VanillaJS.

Puedes encontrar la configuración que soporta el componente de Vue <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">aquí</a>.