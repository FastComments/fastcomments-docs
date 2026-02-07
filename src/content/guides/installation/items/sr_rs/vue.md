Да бисте додали коментаре на ваш сајт изграђен са Vue, нашу Vue библиотеку на NPM можете пронаћи <a href="https://www.npmjs.com/package/fastcomments-vue" target="_blank">овде</a>.

Такође, библиотека vue-next је на NPM <a href="https://www.npmjs.com/package/fastcomments-vue-next" target="_blank">овде</a>

Изворни код можете пронаћи на <a href="https://github.com/FastComments" target="_blank">GitHub</a>.

FastComments Vue коментарски видгет подржава све исте функције као и VanillaJS верзија - коментарисање у реалном времену, sso, и слично.

Доње инструкције су за Vue 3 пошто је већ дуже време доступан, међутим FastComments такође подржава Vue 2 преко `fastcomments-vue` библиотеке.

[inline-code-attrs-start title = 'FastComments Vue преко NPM'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install --save fastcomments-vue-next
[inline-code-end]


[inline-code-attrs-start title = 'FastComments Vue преко Yarn'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
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

Ако се налазите у ЕУ, желећете да подесите `region` на `EU`:

[inline-code-attrs-start title = 'FastComments Vue - ЕУ'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<fast-comments v-bind:config="{tenantId: 'demo', region: 'eu'}"/>
[inline-code-end]

The `fastcomments-vue` and `fastcomments-vue-next` libraries support the same configuration as the VanillaJS commenting widget.

Конфигурацију коју Vue компонента подржава можете пронаћи <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">овде</a>.

---