Чтобы добавить комментарии на ваш сайт, созданный с помощью Vue, вы можете найти нашу библиотеку Vue на NPM <a href="https://www.npmjs.com/package/fastcomments-vue" target="_blank">here</a>.

Кроме того, библиотека vue-next доступна на NPM <a href="https://www.npmjs.com/package/fastcomments-vue-next" target="_blank">here</a>

Исходный код доступен на <a href="https://github.com/FastComments" target="_blank">GitHub</a>.

Виджет комментариев FastComments для Vue поддерживает все те же функции, что и для VanillaJS — живые комментарии, SSO и т. д.

Ниже инструкции для Vue 3, так как он уже некоторое время доступен, однако FastComments также поддерживает Vue 2 через библиотеку `fastcomments-vue`.

[inline-code-attrs-start title = 'FastComments Vue через NPM'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install --save fastcomments-vue-next
[inline-code-end]


[inline-code-attrs-start title = 'FastComments Vue через Yarn'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
yarn add fastcomments-vue-next
[inline-code-end]


[inline-code-attrs-start title = 'Пример Vue'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

Если вы находитесь в ЕС, вам следует установить `region` в `EU`:

[inline-code-attrs-start title = 'FastComments Vue - ЕС'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<fast-comments v-bind:config="{tenantId: 'demo', region: 'eu'}"/>
[inline-code-end]

Библиотеки `fastcomments-vue` и `fastcomments-vue-next` поддерживают ту же конфигурацию, что и виджет комментариев VanillaJS.

Конфигурацию, которую поддерживает Vue-компонент, можно найти <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">here</a>.