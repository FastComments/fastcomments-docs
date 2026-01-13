Ви можете знайти нашу бібліотеку Vue на NPM <a href="https://www.npmjs.com/package/fastcomments-vue" target="_blank">тут</a>.

Крім того, бібліотека vue-next доступна на NPM <a href="https://www.npmjs.com/package/fastcomments-vue-next" target="_blank">тут</a>

Вихідний код можна знайти на <a href="https://github.com/FastComments" target="_blank">GitHub</a>.

Віджет коментарів FastComments для Vue підтримує всі ті ж функції, що й VanillaJS версія — коментування в реальному часі, SSO тощо.

Наведені нижче інструкції призначені для Vue 3, оскільки він вже давно випущений, однак FastComments також підтримує Vue 2 через бібліотеку `fastcomments-vue`.

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

Якщо ви знаходитесь у ЄС, вам потрібно встановити `region` у значення `EU`:

[inline-code-attrs-start title = 'FastComments Vue - ЄС'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<fast-comments v-bind:config="{tenantId: 'demo', region: 'eu'}"/>
[inline-code-end]

Бібліотеки `fastcomments-vue` та `fastcomments-vue-next` підтримують ту ж конфігурацію, що й VanillaJS віджет коментарів.

Ви можете знайти конфігурацію, яку підтримує компонент Vue <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">тут</a>.
