```js
import { FastCommentsLiveChatWidget } from 'fastcomments-react-native';

// ...

  const myTenantId = 'demo'; // Seu tenant id. Pode ser obtido em https://fastcomments.com/auth/my-account/api-secret
  const myAppPageId = 'native-test'; // o ID ou URL do tópico de comentários no seu app.
  const myAppPageUrl = 'https://example.com/external-page'; // você pode opcionalmente definir uma url para uma página externa
  const myAppPageTitle = 'Example Title'; // ... e você provavelmente vai querer um título para este conteúdo
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