```js
import { FastCommentsLiveChatWidget } from 'fastcomments-react-native';

// ...

  const myTenantId = 'demo'; // 您的租户 id。可以从 https://fastcomments.com/auth/my-account/api-secret 获取
  const myAppPageId = 'native-test'; // 在您的应用中评论线程的 ID 或 URL。
  const myAppPageUrl = 'https://example.com/external-page'; // 您可以可选地设置指向外部页面的 url
  const myAppPageTitle = 'Example Title'; // ... 并且您可能希望为此内容设置一个标题
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