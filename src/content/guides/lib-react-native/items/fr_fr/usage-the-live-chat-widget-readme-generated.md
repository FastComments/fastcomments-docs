```js
import { FastCommentsLiveChatWidget } from 'fastcomments-react-native';

// ...

  const myTenantId = 'demo'; // Votre tenant id. Peut être récupéré depuis https://fastcomments.com/auth/my-account/api-secret
  const myAppPageId = 'native-test'; // L'ID ou l'URL du fil de commentaires dans votre application.
  const myAppPageUrl = 'https://example.com/external-page'; // vous pouvez éventuellement définir une url vers une page externe
  const myAppPageTitle = 'Example Title'; // ... et vous voudrez probablement un titre pour ce contenu
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