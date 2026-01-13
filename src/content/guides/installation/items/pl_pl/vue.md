Naszą bibliotekę Vue możesz znaleźć na NPM <a href="https://www.npmjs.com/package/fastcomments-vue" target="_blank">tutaj</a>.

Dodatkowo biblioteka vue-next jest dostępna na NPM <a href="https://www.npmjs.com/package/fastcomments-vue-next" target="_blank">tutaj</a>

Kod źródłowy można znaleźć na <a href="https://github.com/FastComments" target="_blank">GitHub</a>.

Widget komentarzy FastComments dla Vue obsługuje wszystkie te same funkcje co wersja VanillaJS - komentarze na żywo, SSO i więcej.

Poniższe instrukcje dotyczą Vue 3, ponieważ jest dostępne od jakiegoś czasu, jednak FastComments obsługuje również Vue 2 poprzez bibliotekę `fastcomments-vue`.

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

Jeśli jesteś w UE, będziesz chciał ustawić `region` na `EU`:

[inline-code-attrs-start title = 'FastComments Vue - EU'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<fast-comments v-bind:config="{tenantId: 'demo', region: 'eu'}"/>
[inline-code-end]

Biblioteki `fastcomments-vue` i `fastcomments-vue-next` obsługują tę samą konfigurację co widget komentarzy VanillaJS.

Konfigurację obsługiwaną przez komponent Vue możesz znaleźć <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">tutaj</a>.
