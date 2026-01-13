The API is slightly different compared to `fastcomments-react`. With the native package, we pass a config object that follows [this structure](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L35).

```js
import { FastCommentsCommentWidget } from 'fastcomments-react-native';

// ...

  const myTenantId = 'demo'; // Your tenant ID. Can be fetched from https://fastcomments.com/auth/my-account/api-secret
  const myAppPageId = 'native-test'; // The ID or URL of the comment thread in your app.
  const myAppPageUrl = 'https://example.com/external-page'; // You can optionally set a URL to an external page
  const myAppPageTitle = 'Example Title'; // ... and you'll probably want a title for this content
  const [config, setConfig] = useState({
    tenantId: myTenantId,
    urlId: myAppPageId,
    url: myAppPageUrl,
    pageTitle: myAppPageTitle
  });

  // By calling setConfig(), you can do things like change the current page or the currently logged-in user
  // See example/src/App.tsx

  return (
      <FastCommentsCommentWidget config={config}/>
  );
```