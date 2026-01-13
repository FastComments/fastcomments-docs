---
The APIは`fastcomments-react`と比べて若干異なります。ネイティブでは、次の構造に従う設定オブジェクトを渡します: [this structure](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L35)。

```js
import { FastCommentsCommentWidget } from 'fastcomments-react-native';

// ...

  const myTenantId = 'demo'; // あなたのテナントID。https://fastcomments.com/auth/my-account/api-secret から取得できます
  const myAppPageId = 'native-test'; // アプリ内のコメントスレッドのIDまたはURL。
  const myAppPageUrl = 'https://example.com/external-page'; // 外部ページのURLを任意で設定できます
  const myAppPageTitle = 'Example Title'; // そして、おそらくこのコンテンツのタイトルが必要でしょう
  const [config, setConfig] = useState({
    tenantId: myTenantId,
    urlId: myAppPageId,
    url: myAppPageUrl,
    pageTitle: myAppPageTitle
  });

  // by calling setConfig(), we can do things like change the current page, or the currently logged in user
  // example/src/App.tsx を参照してください

  return (
      <FastCommentsCommentWidget config={config}/>
  );
```
---