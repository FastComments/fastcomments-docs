כדי להוסיף תגובות לאתר שבנית באמצעות Vue, ניתן למצוא את ספריית Vue שלנו ב‑NPM <a href="https://www.npmjs.com/package/fastcomments-vue" target="_blank">כאן</a>.

בנוסף, ספריית vue-next נמצאת ב‑NPM <a href="https://www.npmjs.com/package/fastcomments-vue-next" target="_blank">כאן</a>

קוד המקור זמין ב‑<a href="https://github.com/FastComments" target="_blank">GitHub</a>.

ווידג'ט התגובות של FastComments ל‑Vue תומך בכל אותן התכונות כמו של ה‑VanillaJS — תגובות בזמן אמת, SSO, וכן הלאה.

ההוראות הבאות מיועדות ל‑Vue 3 מכיוון שיצאה מזה זמן, עם זאת FastComments תומכת גם ב‑Vue 2 באמצעות הספרייה `fastcomments-vue`.

[inline-code-attrs-start title = 'FastComments Vue באמצעות NPM'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install --save fastcomments-vue-next
[inline-code-end]


[inline-code-attrs-start title = 'FastComments Vue באמצעות Yarn'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
yarn add fastcomments-vue-next
[inline-code-end]


[inline-code-attrs-start title = 'דוגמה ל‑Vue'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

אם אתם באיחוד האירופי, רצוי להגדיר את `region` כ‑`EU`:

[inline-code-attrs-start title = 'FastComments Vue - אזור EU'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<fast-comments v-bind:config="{tenantId: 'demo', region: 'eu'}"/>
[inline-code-end]

הספריות `fastcomments-vue` ו‑`fastcomments-vue-next` תומכות באותה תצורה כמו ווידג'ט התגובות של VanillaJS.

ניתן למצוא את התצורה שהקומפוננטה של Vue תומכת בה <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">כאן</a>.