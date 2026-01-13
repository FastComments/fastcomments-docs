API'en er en smule anderledes sammenlignet med `fastcomments-react`. Med native sender vi et config-objekt, som følger [denne struktur](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L35).

```js
import { FastCommentsCommentWidget } from 'fastcomments-react-native';

// ...

  const myTenantId = 'demo'; // Dit tenant-id. Kan hentes fra https://fastcomments.com/auth/my-account/api-secret
  const myAppPageId = 'native-test'; // ID'et eller URL'en for kommentartråden i din app.
  const myAppPageUrl = 'https://example.com/external-page'; // du kan valgfrit angive en URL til en ekstern side
  const myAppPageTitle = 'Example Title'; // ... og du vil sandsynligvis gerne have en titel til dette indhold
  const [config, setConfig] = useState({
    tenantId: myTenantId,
    urlId: myAppPageId,
    url: myAppPageUrl,
    pageTitle: myAppPageTitle
  });

  // Ved at kalde setConfig() kan vi for eksempel ændre den aktuelle side eller den aktuelt loggede bruger
  // Se example/src/App.tsx

  return (
      <FastCommentsCommentWidget config={config}/>
  );
```