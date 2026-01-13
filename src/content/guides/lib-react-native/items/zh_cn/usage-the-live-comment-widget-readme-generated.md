与 `fastcomments-react` 相比，API 略有不同。在原生环境中，我们传入一个遵循 [此结构](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L35) 的配置对象。

```js
import { FastCommentsCommentWidget } from 'fastcomments-react-native';

// ...

  const myTenantId = 'demo'; // 您的租户 ID。可以从 https://fastcomments.com/auth/my-account/api-secret 获取
  const myAppPageId = 'native-test'; // 您应用中评论线程的 ID 或 URL。
  const myAppPageUrl = 'https://example.com/external-page'; // 您可以可选地为外部页面设置一个 URL
  const myAppPageTitle = 'Example Title'; // … 并且您可能想为此内容设置标题
  const [config, setConfig] = useState({
    tenantId: myTenantId,
    urlId: myAppPageId,
    url: myAppPageUrl,
    pageTitle: myAppPageTitle
  });

  // 通过调用 setConfig()，我们可以做诸如更改当前页面或当前登录用户之类的操作
  // 参见 example/src/App.tsx

  return (
      <FastCommentsCommentWidget config={config}/>
  );
```