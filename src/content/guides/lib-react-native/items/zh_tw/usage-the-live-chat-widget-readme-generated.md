```js
import { FastCommentsLiveChatWidget } from 'fastcomments-react-native';

// ...

  const myTenantId = 'demo'; // 您的租戶 ID。可以從 https://fastcomments.com/auth/my-account/api-secret 取得
  const myAppPageId = 'native-test'; // 您應用中留言串的 ID 或 URL。
  const myAppPageUrl = 'https://example.com/external-page'; // 您可以選擇為外部頁面設定 url
  const myAppPageTitle = 'Example Title'; // …並且您可能會想為此內容設定標題
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