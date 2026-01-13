```js
import { FastCommentsLiveChatWidget } from 'fastcomments-react-native';

// ...

  const myTenantId = 'demo'; // ID вашего тенанта. Можно получить с https://fastcomments.com/auth/my-account/api-secret
  const myAppPageId = 'native-test'; // ID или URL потока комментариев в вашем приложении.
  const myAppPageUrl = 'https://example.com/external-page'; // При желании можно указать URL внешней страницы
  const myAppPageTitle = 'Example Title'; // ... и, вероятно, вам понадобится заголовок для этого контента
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