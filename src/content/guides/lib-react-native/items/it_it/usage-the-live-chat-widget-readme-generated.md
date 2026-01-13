```js
import { FastCommentsLiveChatWidget } from 'fastcomments-react-native';

// ...

  const myTenantId = 'demo'; // Il tuo tenant id. Può essere recuperato da https://fastcomments.com/auth/my-account/api-secret
  const myAppPageId = 'native-test'; // l'ID o l'URL del thread di commenti nella tua app.
  const myAppPageUrl = 'https://example.com/external-page'; // puoi opzionalmente impostare un URL verso una pagina esterna
  const myAppPageTitle = 'Example Title'; // ... e probabilmente vorrai un titolo per questo contenuto
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