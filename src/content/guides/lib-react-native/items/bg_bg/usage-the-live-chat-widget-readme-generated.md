```js
import { FastCommentsLiveChatWidget } from 'fastcomments-react-native';

// ...

  const myTenantId = 'demo'; // Идентификаторът (tenant id) за вашия акаунт. Може да бъде получен от https://fastcomments.com/auth/my-account/api-secret
  const myAppPageId = 'native-test'; // ID-то или URL-то на нишката с коментари във вашето приложение.
  const myAppPageUrl = 'https://example.com/external-page'; // По желание можете да зададете URL към външна страница
  const myAppPageTitle = 'Example Title'; // ... и вероятно ще искате заглавие за това съдържание
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