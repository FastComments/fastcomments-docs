Για να προσθέσετε σχόλια στην εφαρμογή React Native, μπορείτε να βρείτε τη βιβλιοθήκη React Native στο NPM <a href="https://www.npmjs.com/package/fastcomments-react-native" target="_blank">εδώ</a>.

Το widget σχολιασμού FastComments για React Native υποστηρίζει όλες τις ίδιες δυνατότητες με αυτό του VanillaJS - live commenting, sso, και τα λοιπά.

[inline-code-attrs-start title = 'FastComments React Native μέσω NPM'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install --save fastcomments-react-native
[inline-code-end]


[inline-code-attrs-start title = 'FastComments React Native μέσω Yarn'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
yarn add fastcomments-react-native
[inline-code-end]

Η διαμόρφωση καθορίζεται ελαφρώς διαφορετικά σε σύγκριση με τη βιβλιοθήκη `fastcomments-react`:

[inline-code-attrs-start title = 'Παράδειγμα React Native'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import { FastCommentsCommentWidget } from 'fastcomments-react-native';

// ...

  const tenantId = 'demo'; // Το tenant id σας. Μπορεί να αντληθεί από https://fastcomments.com/auth/my-account/api-secret
  const pageId = 'native-test'; // Το ID ή το URL του νήματος σχολίων στην εφαρμογή σας.
  const config = {
    tenantId: tenantId,
    urlId: pageId
  };

  return (
      <FastCommentsCommentWidget config={config}/>
  );
[inline-code-end]

Αν βρίσκεστε στην ΕΕ, θα θέλετε να ορίσετε την παράμετρο `region`:

[inline-code-attrs-start title = 'React Native - ΕΕ'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
  // ...
  const config = {
    tenantId: tenantId,
    urlId: pageId,
    region: 'eu'
  };
  // ...
[inline-code-end]

Μπορείτε να βρείτε τη διαμόρφωση που υποστηρίζει το component React Native <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">εδώ</a>.

---