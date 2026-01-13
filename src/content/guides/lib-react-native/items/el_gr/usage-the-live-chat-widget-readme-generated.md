```js
import { FastCommentsLiveChatWidget } from 'fastcomments-react-native';

// ...

  const myTenantId = 'demo'; // Το tenant id σας. Μπορεί να ανακτηθεί από https://fastcomments.com/auth/my-account/api-secret
  const myAppPageId = 'native-test'; // Το ID ή το URL του νήματος σχολίων στην εφαρμογή σας.
  const myAppPageUrl = 'https://example.com/external-page'; // Μπορείτε προαιρετικά να ορίσετε ένα url προς μια εξωτερική σελίδα
  const myAppPageTitle = 'Example Title'; // ... και πιθανότατα θέλετε έναν τίτλο για αυτό το περιεχόμενο
  const config = {
    tenantId: myTenantId,
    urlId: myAppPageId,
    url: myAppPageUrl,
    pageTitle: myAppPageTitle
  };

  return (
      <FastCommentsLiveChatWidget config={config}/>
  );
```