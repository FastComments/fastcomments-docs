Για να προσθέσετε σχόλια στον ιστότοπό σας που έχει κατασκευαστεί με Vue, μπορείτε να βρείτε τη βιβλιοθήκη μας για Vue στο NPM <a href="https://www.npmjs.com/package/fastcomments-vue" target="_blank">εδώ</a>.

Επιπλέον, η βιβλιοθήκη vue-next είναι στο NPM <a href="https://www.npmjs.com/package/fastcomments-vue-next" target="_blank">εδώ</a>

Ο πηγαίος κώδικας βρίσκεται στο <a href="https://github.com/FastComments" target="_blank">GitHub</a>.

Το widget σχολιασμού FastComments για Vue υποστηρίζει όλες τις ίδιες δυνατότητες με αυτό του VanillaJS - ζωντανά σχόλια, sso, κ.λπ.

Οι παρακάτω οδηγίες αφορούν το Vue 3 αφού είναι διαθέσιμο εδώ και καιρό, ωστόσο το FastComments υποστηρίζει επίσης το Vue 2 μέσω της βιβλιοθήκης `fastcomments-vue`.

[inline-code-attrs-start title = 'FastComments Vue μέσω NPM'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install --save fastcomments-vue-next
[inline-code-end]


[inline-code-attrs-start title = 'FastComments Vue μέσω Yarn'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
yarn add fastcomments-vue-next
[inline-code-end]


[inline-code-attrs-start title = 'Παράδειγμα Vue'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

If you're in the EU, you'll want to set the `region` to `EU`:

[inline-code-attrs-start title = 'FastComments Vue - ΕΕ'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<fast-comments v-bind:config="{tenantId: 'demo', region: 'eu'}"/>
[inline-code-end]

The `fastcomments-vue` and `fastcomments-vue-next` libraries support the same configuration as the VanillaJS commenting widget.

Μπορείτε να βρείτε τη διαμόρφωση που υποστηρίζει το συστατικό Vue <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">εδώ</a>.

---