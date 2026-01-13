```js
import { FastCommentsLiveChatWidget } from 'fastcomments-react-native';

// ...

  const myTenantId = 'demo'; // Your tenant ID. Can be fetched from https://fastcomments.com/auth/my-account/api-secret
  const myAppPageId = 'native-test'; // The ID or URL of the comment thread in your app.
  const myAppPageUrl = 'https://example.com/external-page'; // You can optionally set a URL to an external page
  const myAppPageTitle = 'Example Title'; // ... and you probably want a title for this content
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