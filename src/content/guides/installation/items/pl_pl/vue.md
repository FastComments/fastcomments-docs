Aby dodać komentarze do swojej strony zbudowanej w Vue, możesz znaleźć naszą bibliotekę Vue na NPM <a href="https://www.npmjs.com/package/fastcomments-vue" target="_blank">tutaj</a>.

Dodatkowo biblioteka vue-next jest dostępna na NPM <a href="https://www.npmjs.com/package/fastcomments-vue-next" target="_blank">tutaj</a>

Kod źródłowy można znaleźć na <a href="https://github.com/FastComments" target="_blank">GitHub</a>.

Widget komentowania FastComments Vue obsługuje wszystkie te same funkcje co wersja VanillaJS - live commenting, sso, i tak dalej.

Poniższe instrukcje dotyczą Vue 3, ponieważ jest dostępne od jakiegoś czasu, jednak FastComments obsługuje również Vue 2 poprzez bibliotekę `fastcomments-vue`.

[inline-code-attrs-start title = 'FastComments Vue za pośrednictwem NPM'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install --save fastcomments-vue-next
[inline-code-end]


[inline-code-attrs-start title = 'FastComments Vue za pośrednictwem Yarn'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
yarn add fastcomments-vue-next
[inline-code-end]


[inline-code-attrs-start title = 'Przykład Vue'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

Jeśli znajdujesz się w UE, ustaw `region` na `EU`:

[inline-code-attrs-start title = 'FastComments Vue - UE'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<fast-comments v-bind:config="{tenantId: 'demo', region: 'eu'}"/>
[inline-code-end]

Biblioteki `fastcomments-vue` i `fastcomments-vue-next` obsługują tę samą konfigurację co widget komentujący VanillaJS.

Możesz znaleźć konfigurację, którą obsługuje komponent Vue, <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">tutaj</a>.