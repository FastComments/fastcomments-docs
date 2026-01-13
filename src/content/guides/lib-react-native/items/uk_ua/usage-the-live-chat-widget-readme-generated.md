```js
import { FastCommentsLiveChatWidget } from 'fastcomments-react-native';

// ...

  const myTenantId = 'demo'; // ID вашого tenant'а. Можна отримати з https://fastcomments.com/auth/my-account/api-secret
  const myAppPageId = 'native-test'; // ID або URL потоку коментарів у вашому додатку.
  const myAppPageUrl = 'https://example.com/external-page'; // Ви можете опціонально вказати URL зовнішньої сторінки
  const myAppPageTitle = 'Example Title'; // ... і, ймовірно, вам знадобиться заголовок для цього вмісту
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