```js
import { FastCommentsLiveChatWidget } from 'fastcomments-react-native';

// ...

  const myTenantId = 'demo'; // Tu tenant ID. Puede obtenerse desde https://fastcomments.com/auth/my-account/api-secret
  const myAppPageId = 'native-test'; // El ID o URL del hilo de comentarios en tu app.
  const myAppPageUrl = 'https://example.com/external-page'; // puedes configurar opcionalmente una URL a una página externa
  const myAppPageTitle = 'Example Title'; // ... y probablemente querrás un título para este contenido
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