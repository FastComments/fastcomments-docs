```js
import { FastCommentsLiveChatWidget } from 'fastcomments-react-native';

// ...

  const myTenantId = 'demo'; // Ваш идентификатор тенанта. Може се преузети са https://fastcomments.com/auth/my-account/api-secret
  const myAppPageId = 'native-test'; // ИД или URL теме коментара у вашој апликацији.
  const myAppPageUrl = 'https://example.com/external-page'; // Можете опционо поставити URL ка екстерној страници
  const myAppPageTitle = 'Example Title'; // ... и највероватније ћете желети наслов за овај садржај
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