Το API είναι ελαφρώς διαφορετικό σε σύγκριση με `fastcomments-react`. Στην native έκδοση, περνάμε ένα αντικείμενο config το οποίο ακολουθεί [αυτή τη δομή](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L35).

```js
import { FastCommentsCommentWidget } from 'fastcomments-react-native';

// ...

  const myTenantId = 'demo'; // Το tenant id σας. Μπορεί να ανακτηθεί από https://fastcomments.com/auth/my-account/api-secret
  const myAppPageId = 'native-test'; // Το ID ή το URL του νήματος σχολίων στην εφαρμογή σας.
  const myAppPageUrl = 'https://example.com/external-page'; // μπορείτε προαιρετικά να ορίσετε ένα URL σε εξωτερική σελίδα
  const myAppPageTitle = 'Example Title'; // ... και πιθανότατα θέλετε έναν τίτλο για αυτό το περιεχόμενο
  const [config, setConfig] = useState({
    tenantId: myTenantId,
    urlId: myAppPageId,
    url: myAppPageUrl,
    pageTitle: myAppPageTitle
  });

  // καλώντας το setConfig(), μπορούμε να κάνουμε πράγματα όπως να αλλάξουμε την τρέχουσα σελίδα ή τον τρέχοντα συνδεδεμένο χρήστη
  // Δείτε example/src/App.tsx

  return (
      <FastCommentsCommentWidget config={config}/>
  );
```