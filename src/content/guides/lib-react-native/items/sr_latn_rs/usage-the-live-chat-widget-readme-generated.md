```js
import { FastCommentsLiveChatWidget } from 'fastcomments-react-native';

// ...

  const myTenantId = 'demo'; // Vaš tenant id. Može se preuzeti sa https://fastcomments.com/auth/my-account/api-secret
  const myAppPageId = 'native-test'; // ID ili URL nita komentara u vašoj aplikaciji.
  const myAppPageUrl = 'https://example.com/external-page'; // Neobavezno možete postaviti URL ka eksternoj stranici
  const myAppPageTitle = 'Example Title'; // ... i verovatno ćete želeti naslov za ovaj sadržaj
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