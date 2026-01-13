與 `fastcomments-react` 相比，這個 API 有些不同。在原生（native）中，我們傳入一個遵循 [此結構](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L35) 的 config 物件。

```js
import { FastCommentsCommentWidget } from 'fastcomments-react-native';

// ...

  const myTenantId = 'demo'; // 您的 tenant id。可以從 https://fastcomments.com/auth/my-account/api-secret 取得
  const myAppPageId = 'native-test'; // 您應用程式中討論串的 ID 或 URL。
  const myAppPageUrl = 'https://example.com/external-page'; // 您可以選擇設定一個外部頁面的 URL
  const myAppPageTitle = 'Example Title'; // ... 並且您可能想為此內容設定一個標題
  const [config, setConfig] = useState({
    tenantId: myTenantId,
    urlId: myAppPageId,
    url: myAppPageUrl,
    pageTitle: myAppPageTitle
  });

  // 透過呼叫 setConfig()，我們可以做像是更換當前頁面或目前登入的使用者之類的操作
  // 參見 example/src/App.tsx

  return (
      <FastCommentsCommentWidget config={config}/>
  );
```