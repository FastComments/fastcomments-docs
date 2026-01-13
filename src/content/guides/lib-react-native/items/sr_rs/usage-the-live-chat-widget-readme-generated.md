```js
import { FastCommentsLiveChatWidget } from 'fastcomments-react-native';

// ...

  const myTenantId = 'demo'; // Ваш tenant id. Може се преузети са https://fastcomments.com/auth/my-account/api-secret
  const myAppPageId = 'native-test'; // ID или URL нити коментара у вашој апликацији.
  const myAppPageUrl = 'https://example.com/external-page'; // можете опционално поставити url ка спољној страници
  const myAppPageTitle = 'Example Title'; // ... и вероватно ћете желети наслов за овај садржај
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