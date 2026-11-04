Да бисте додали коментаре на свој веб сајт направљен помоћу Vue-а, нашу Vue библиотеку можете пронаћи на NPM <a href="https://www.npmjs.com/package/fastcomments-vue" target="_blank">овде</a>.

Поред тога, vue-next библиотека се налази на NPM <a href="https://www.npmjs.com/package/fastcomments-vue-next" target="_blank">овде</a>

Изворни код можете пронаћи на <a href="https://github.com/FastComments" target="_blank">GitHub-у</a>.

FastComments Vue виџет за коментаре подржава све исте функције као VanillaJS верзија - коментаре у реалном времену, sso и тако даље.

Упутства у наставку важе за Vue 3 јер је ту већ неко време, међутим FastComments такође подржава Vue 2 преко `fastcomments-vue` библиотеке.

[inline-code-attrs-start title = 'FastComments Vue преко NPM-а'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install --save fastcomments-vue-next
[inline-code-end]


[inline-code-attrs-start title = 'FastComments Vue преко Yarn-а'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
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

Ако сте у ЕУ, желите да подесите `region` на `EU`:

[inline-code-attrs-start title = 'FastComments Vue - ЕУ'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<fast-comments v-bind:config="{tenantId: 'demo', region: 'eu'}"/>
[inline-code-end]

Библиотеке `fastcomments-vue` и `fastcomments-vue-next` подржавају исту конфигурацију као VanillaJS виџет за коментаре.

Конфигурацију коју Vue компонента подржава можете пронаћи <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">овде</a>.