---
L'API est légèrement différente comparée à `fastcomments-react`. Avec la version native, nous passons un objet de configuration qui suit [cette structure](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L35).

```js
import { FastCommentsCommentWidget } from 'fastcomments-react-native';

// ...

  const myTenantId = 'demo'; // Votre tenant id. Peut être récupéré depuis https://fastcomments.com/auth/my-account/api-secret
  const myAppPageId = 'native-test'; // L'ID ou l'URL du fil de commentaires dans votre application.
  const myAppPageUrl = 'https://example.com/external-page'; // Vous pouvez éventuellement définir une url vers une page externe
  const myAppPageTitle = 'Example Title'; // ... et vous voudrez probablement un titre pour ce contenu
  const [config, setConfig] = useState({
    tenantId: myTenantId,
    urlId: myAppPageId,
    url: myAppPageUrl,
    pageTitle: myAppPageTitle
  });

  // en appelant setConfig(), on peut, par exemple, changer la page courante ou l'utilisateur actuellement connecté
  // Voir example/src/App.tsx

  return (
      <FastCommentsCommentWidget config={config}/>
  );
```
---