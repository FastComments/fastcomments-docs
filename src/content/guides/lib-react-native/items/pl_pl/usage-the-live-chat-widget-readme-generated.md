```js
import { FastCommentsLiveChatWidget } from 'fastcomments-react-native';

// ...

  const myTenantId = 'demo'; // Twój tenant id. Można go pobrać z https://fastcomments.com/auth/my-account/api-secret
  const myAppPageId = 'native-test'; // ID lub URL wątku komentarzy w Twojej aplikacji.
  const myAppPageUrl = 'https://example.com/external-page'; // możesz opcjonalnie ustawić URL do zewnętrznej strony
  const myAppPageTitle = 'Example Title'; // ... i prawdopodobnie będziesz chciał tytuł dla tej treści
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