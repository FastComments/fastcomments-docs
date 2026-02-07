Щоб додати коментарі на ваш сайт, побудований із Vue, ви можете знайти нашу бібліотеку Vue на NPM <a href="https://www.npmjs.com/package/fastcomments-vue" target="_blank">тут</a>.

Крім того, бібліотека vue-next доступна на NPM <a href="https://www.npmjs.com/package/fastcomments-vue-next" target="_blank">тут</a>

Вихідний код можна знайти на <a href="https://github.com/FastComments" target="_blank">GitHub</a>.

Компонент коментарів FastComments для Vue підтримує всі ті ж функції, що й VanillaJS — живі коментарі, sso тощо.

Нижченаведені інструкції призначені для Vue 3, оскільки він був доступний деякий час; однак FastComments також підтримує Vue 2 через бібліотеку `fastcomments-vue`.

[inline-code-attrs-start title = 'FastComments Vue через NPM'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install --save fastcomments-vue-next
[inline-code-end]


[inline-code-attrs-start title = 'FastComments Vue через Yarn'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
yarn add fastcomments-vue-next
[inline-code-end]


[inline-code-attrs-start title = 'Приклад Vue'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

Якщо ви в ЄС, вам слід встановити `region` в `EU`:

[inline-code-attrs-start title = 'FastComments Vue - ЄС'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<fast-comments v-bind:config="{tenantId: 'demo', region: 'eu'}"/>
[inline-code-end]

Бібліотеки `fastcomments-vue` та `fastcomments-vue-next` підтримують ту саму конфігурацію, що й віджет коментарів VanillaJS.

Ви можете знайти конфігурацію, яку підтримує компонент Vue, <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">тут</a>.

---