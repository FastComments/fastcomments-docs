```js
import { FastCommentsLiveChatWidget } from 'fastcomments-react-native';

// ...

  const myTenantId = 'demo'; // Το tenant id σας. Μπορεί να ανακτηθεί από https://fastcomments.com/auth/my-account/api-secret
  const myAppPageId = 'native-test'; // Το ID ή το URL του νήματος σχολίων στην εφαρμογή σας.
  const myAppPageUrl = 'https://example.com/external-page'; // μπορείτε προαιρετικά να ορίσετε ένα URL προς εξωτερική σελίδα
  const myAppPageTitle = 'Example Title'; // ... και πιθανότατα θα θέλατε έναν τίτλο για αυτό το περιεχόμενο
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