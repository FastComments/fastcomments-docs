Μπορείτε να βρείτε τη βιβλιοθήκη Vue μας στο NPM <a href="https://www.npmjs.com/package/fastcomments-vue" target="_blank">εδώ</a>.

Επιπλέον, μια βιβλιοθήκη vue-next είναι στο NPM <a href="https://www.npmjs.com/package/fastcomments-vue-next" target="_blank">εδώ</a>

Ο πηγαίος κώδικας μπορεί να βρεθεί στο <a href="https://github.com/FastComments" target="_blank">GitHub</a>.

Το FastComments Vue widget σχολίων υποστηρίζει όλες τις ίδιες δυνατότητες με την έκδοση VanillaJS - ζωντανά σχόλια, SSO και ούτω καθεξής.

Οι παρακάτω οδηγίες είναι για Vue 3 δεδομένου ότι έχει κυκλοφορήσει εδώ και καιρό, ωστόσο το FastComments υποστηρίζει επίσης Vue 2 μέσω της βιβλιοθήκης `fastcomments-vue`.

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

Αν είστε στην EU, θα θέλετε να ορίσετε το `region` σε `EU`:

[inline-code-attrs-start title = 'FastComments Vue - EU'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<fast-comments v-bind:config="{tenantId: 'demo', region: 'eu'}"/>
[inline-code-end]

Οι βιβλιοθήκες `fastcomments-vue` και `fastcomments-vue-next` υποστηρίζουν την ίδια διαμόρφωση με το VanillaJS widget σχολίων.

Μπορείτε να βρείτε τη διαμόρφωση που υποστηρίζει το Vue component <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">εδώ</a>.
