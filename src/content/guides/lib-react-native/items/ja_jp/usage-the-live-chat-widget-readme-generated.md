```js
import { FastCommentsLiveChatWidget } from 'fastcomments-react-native';

// ...

  const myTenantId = 'demo'; // あなたのテナントID。https://fastcomments.com/auth/my-account/api-secret から取得できます
  const myAppPageId = 'native-test'; // アプリ内のコメントスレッドのIDまたはURL。
  const myAppPageUrl = 'https://example.com/external-page'; // 外部ページのurlを任意で設定できます
  const myAppPageTitle = 'Example Title'; // ...そしておそらくこのコンテンツのタイトルを設定したいでしょう
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