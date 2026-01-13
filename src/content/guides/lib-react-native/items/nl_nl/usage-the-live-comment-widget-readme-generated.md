---
De API verschilt iets in vergelijking met `fastcomments-react`. Bij native geven we een config-object door dat de [volgende structuur](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L35) volgt.

```js
import { FastCommentsCommentWidget } from 'fastcomments-react-native';

// ...

  const myTenantId = 'demo'; // Uw tenant-id. Kan worden opgehaald van https://fastcomments.com/auth/my-account/api-secret
  const myAppPageId = 'native-test'; // De ID of URL van de commentdraad in uw app.
  const myAppPageUrl = 'https://example.com/external-page'; // U kunt optioneel een URL instellen naar een externe pagina
  const myAppPageTitle = 'Example Title'; // ... en u wilt waarschijnlijk een titel voor deze inhoud
  const [config, setConfig] = useState({
    tenantId: myTenantId,
    urlId: myAppPageId,
    url: myAppPageUrl,
    pageTitle: myAppPageTitle
  });

  // door setConfig() aan te roepen, kunnen we bijvoorbeeld de huidige pagina of de momenteel ingelogde gebruiker wijzigen
  // Zie example/src/App.tsx

  return (
      <FastCommentsCommentWidget config={config}/>
  );
```
---