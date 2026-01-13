The API is slightly different compared to `fastcomments-react`. With native, we pass a config object which follows [this structure](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L35).

```js
import { FastCommentsCommentWidget } from 'fastcomments-react-native';

// ...

  const myTenantId = 'demo'; // Your tenant id. Can be fetched from https://fastcomments.com/auth/my-account/api-secret
  const myAppPageId = 'native-test'; // the ID or URL of the comment thread in your app.
  const myAppPageUrl = 'https://example.com/external-page'; // you can optional set a url to an external page
  const myAppPageTitle = 'Example Title'; // ... and you probably want a title for this content
  const [config, setConfig] = useState({
    tenantId: myTenantId,
    urlId: myAppPageId,
    url: myAppPageUrl,
    pageTitle: myAppPageTitle
  });

  // by calling setConfig(), we can do things like change the current page, or the currently logged in user
  // See example/src/App.tsx

  return (
      <FastCommentsCommentWidget config={config}/>
  );
```