אתה יכול למצוא את ספריית ה-Vue שלנו ב-NPM <a href="https://www.npmjs.com/package/fastcomments-vue" target="_blank">כאן</a>.

בנוסף, ספריית vue-next נמצאת ב-NPM <a href="https://www.npmjs.com/package/fastcomments-vue-next" target="_blank">כאן</a>

קוד המקור ניתן למצוא ב-<a href="https://github.com/FastComments" target="_blank">GitHub</a>.

ווידג'ט התגובות של FastComments ל-Vue תומך בכל אותן התכונות של גרסת VanillaJS - תגובות בזמן אמת, SSO, ועוד.

ההוראות להלן הן עבור Vue 3 מכיוון שהוא יצא כבר זמן מה, עם זאת FastComments תומך גם ב-Vue 2 דרך ספריית `fastcomments-vue`.

[inline-code-attrs-start title = 'FastComments Vue דרך NPM'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install --save fastcomments-vue-next
[inline-code-end]


[inline-code-attrs-start title = 'FastComments Vue דרך Yarn'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
yarn add fastcomments-vue-next
[inline-code-end]


[inline-code-attrs-start title = 'דוגמת Vue'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

אם אתה באיחוד האירופי, תרצה להגדיר את `region` ל-`EU`:

[inline-code-attrs-start title = 'FastComments Vue - EU'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<fast-comments v-bind:config="{tenantId: 'demo', region: 'eu'}"/>
[inline-code-end]

ספריות `fastcomments-vue` ו-`fastcomments-vue-next` תומכות באותן הגדרות כמו ווידג'ט התגובות של VanillaJS.

אתה יכול למצוא את ההגדרות שהקומפוננטה של Vue תומכת בהן <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">כאן</a>.
