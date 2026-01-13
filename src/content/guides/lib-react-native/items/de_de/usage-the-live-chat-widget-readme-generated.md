```js
import { FastCommentsLiveChatWidget } from 'fastcomments-react-native';

// ...

  const myTenantId = 'demo'; // Ihre Tenant-ID. Kann von https://fastcomments.com/auth/my-account/api-secret abgerufen werden
  const myAppPageId = 'native-test'; // Die ID oder URL des Kommentarthreads in Ihrer App.
  const myAppPageUrl = 'https://example.com/external-page'; // Sie können optional eine URL zu einer externen Seite angeben
  const myAppPageTitle = 'Example Title'; // ... und Sie möchten wahrscheinlich einen Titel für diesen Inhalt
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