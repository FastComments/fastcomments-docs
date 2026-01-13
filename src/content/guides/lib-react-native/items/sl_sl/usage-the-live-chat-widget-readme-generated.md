```js
import { FastCommentsLiveChatWidget } from 'fastcomments-react-native';

// ...

  const myTenantId = 'demo'; // ID vašega najemnika. Lahko ga pridobite na https://fastcomments.com/auth/my-account/api-secret
  const myAppPageId = 'native-test'; // ID ali URL niti komentarjev v vaši aplikaciji.
  const myAppPageUrl = 'https://example.com/external-page'; // Neobvezno lahko nastavite URL do zunanje strani
  const myAppPageTitle = 'Example Title'; // ... in verjetno boste želeli naslov za to vsebino
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