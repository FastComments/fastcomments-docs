Du kan finde vores Vue-bibliotek på NPM <a href="https://www.npmjs.com/package/fastcomments-vue" target="_blank">her</a>.

Derudover findes vue-next-biblioteket på NPM <a href="https://www.npmjs.com/package/fastcomments-vue-next" target="_blank">her</a>

[inline-code-attrs-start title = 'FastComments Vue via NPM'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install --save fastcomments-vue-next
[inline-code-end]

[inline-code-attrs-start title = 'Vue eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<template>
    <fast-comments v-bind:config="{tenantId: 'demo'}"/>
</template>
<script>
import {FastComments} from 'fastcomments-vue-next';
export default { name: 'App', components: { FastComments } }
</script>
[inline-code-end]

Du kan finde konfigurationen, som Vue-komponenten understøtter <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">her</a>.
