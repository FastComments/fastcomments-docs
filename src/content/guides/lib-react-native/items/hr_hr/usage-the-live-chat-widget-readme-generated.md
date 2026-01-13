```js
import { FastCommentsLiveChatWidget } from 'fastcomments-react-native';

// ...

  const myTenantId = 'demo'; // Vaš tenant id. Može se dohvatiti s https://fastcomments.com/auth/my-account/api-secret
  const myAppPageId = 'native-test'; // ID ili URL niti komentara u vašoj aplikaciji.
  const myAppPageUrl = 'https://example.com/external-page'; // Opcionalno možete postaviti URL na vanjsku stranicu
  const myAppPageTitle = 'Example Title'; // ... i vjerojatno želite naslov za ovaj sadržaj
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