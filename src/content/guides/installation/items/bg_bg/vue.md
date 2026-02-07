---
За да добавите коментари към вашия уебсайт, изграден с Vue, можете да намерите нашата библиотека за Vue в NPM <a href="https://www.npmjs.com/package/fastcomments-vue" target="_blank">тук</a>.

Освен това библиотека vue-next е налична в NPM <a href="https://www.npmjs.com/package/fastcomments-vue-next" target="_blank">тук</a>

Изходният код може да бъде намерен в <a href="https://github.com/FastComments" target="_blank">GitHub</a>.

Коментарният widget FastComments за Vue поддържа всички същите функции като този за VanillaJS - живо коментиране, SSO и т.н.

Инструкциите по-долу са за Vue 3, тъй като е на пазара от известно време, въпреки това FastComments също поддържа Vue 2 чрез библиотеката `fastcomments-vue`.

[inline-code-attrs-start title = 'FastComments Vue чрез NPM'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install --save fastcomments-vue-next
[inline-code-end]


[inline-code-attrs-start title = 'FastComments Vue чрез Yarn'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
yarn add fastcomments-vue-next
[inline-code-end]


[inline-code-attrs-start title = 'Пример за Vue'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

Ако се намирате в ЕС, ще искате да зададете `region` на `EU`:

[inline-code-attrs-start title = 'FastComments Vue - ЕС'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<fast-comments v-bind:config="{tenantId: 'demo', region: 'eu'}"/>
[inline-code-end]

Библиотеките `fastcomments-vue` и `fastcomments-vue-next` поддържат същата конфигурация като коментарния widget на VanillaJS.

Можете да намерите конфигурацията, която Vue компонентът поддържа, <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">тук</a>.

---