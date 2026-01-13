```js
import { FastCommentsLiveChatWidget } from 'fastcomments-react-native';

// ...

  const myTenantId = 'demo'; // Uw tenant-id. Kan worden opgehaald van https://fastcomments.com/auth/my-account/api-secret
  const myAppPageId = 'native-test'; // De ID of URL van de reactiedraad in uw app.
  const myAppPageUrl = 'https://example.com/external-page'; // U kunt optioneel een URL instellen voor een externe pagina
  const myAppPageTitle = 'Example Title'; // ... en u wilt waarschijnlijk een titel voor deze inhoud
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