Você pode encontrar nossa biblioteca Vue no NPM <a href="https://www.npmjs.com/package/fastcomments-vue" target="_blank">aqui</a>.

Adicionalmente, uma biblioteca vue-next está disponível no NPM <a href="https://www.npmjs.com/package/fastcomments-vue-next" target="_blank">aqui</a>

O código fonte pode ser encontrado no <a href="https://github.com/FastComments" target="_blank">GitHub</a>.

O widget de comentários FastComments para Vue suporta todos os mesmos recursos da versão VanillaJS - comentários ao vivo, SSO e mais.

As instruções abaixo são para Vue 3, pois já está disponível há algum tempo, no entanto FastComments também suporta Vue 2 através da biblioteca `fastcomments-vue`.

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

Se você está na UE, você vai querer definir a `region` para `EU`:

[inline-code-attrs-start title = 'FastComments Vue - EU'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<fast-comments v-bind:config="{tenantId: 'demo', region: 'eu'}"/>
[inline-code-end]

As bibliotecas `fastcomments-vue` e `fastcomments-vue-next` suportam a mesma configuração que o widget de comentários VanillaJS.

Você pode encontrar a configuração que o componente Vue suporta <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">aqui</a>.
