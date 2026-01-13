```js
import { FastCommentsLiveChatWidget } from 'fastcomments-react-native';

// ...

  const myTenantId = 'demo'; // Dit tenant-id. Kan hentes fra https://fastcomments.com/auth/my-account/api-secret
  const myAppPageId = 'native-test'; // ID'et eller URL'en for kommentertråden i din app.
  const myAppPageUrl = 'https://example.com/external-page'; // du kan valgfrit angive en URL til en ekstern side
  const myAppPageTitle = 'Example Title'; // ... og du vil sandsynligvis ønske en titel til dette indhold
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