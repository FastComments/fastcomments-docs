Можете да намерите нашата Vue библиотека в NPM <a href="https://www.npmjs.com/package/fastcomments-vue" target="_blank">тук</a>.

Също така, vue-next библиотеката е в NPM <a href="https://www.npmjs.com/package/fastcomments-vue-next" target="_blank">тук</a>

Изходният код може да бъде намерен в <a href="https://github.com/FastComments" target="_blank">GitHub</a>.

FastComments Vue уиджетът за коментари поддържа всички същите функции като VanillaJS версията — коментиране в реално време, SSO и т.н.

Инструкциите по-долу са за Vue 3, тъй като е на пазара от известно време, но FastComments също поддържа Vue 2 чрез библиотеката `fastcomments-vue`.

[inline-code-attrs-start title = 'FastComments Vue чрез NPM'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install --save fastcomments-vue-next
[inline-code-end]


[inline-code-attrs-start title = 'FastComments Vue чрез Yarn'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
yarn add fastcomments-vue-next
[inline-code-end]


[inline-code-attrs-start title = 'Vue пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

Ако сте в ЕС, ще искате да зададете `region` на `EU`:

[inline-code-attrs-start title = 'FastComments Vue - ЕС'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<fast-comments v-bind:config="{tenantId: 'demo', region: 'eu'}"/>
[inline-code-end]

Библиотеките `fastcomments-vue` и `fastcomments-vue-next` поддържат същата конфигурация като VanillaJS уиджета за коментари.

Можете да намерите конфигурацията, която Vue компонентът поддържа <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">тук</a>.
