You can find our React Native library on NPM <a href="https://www.npmjs.com/package/fastcomments-react-native" target="_blank">here</a>.

The FastComments React Native commenting widget supports all of the same features of the VanillaJS one - live commenting, sso, and so on.

[inline-code-attrs-start title = 'FastComments React Native via NPM'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install --save fastcomments-react-native
[inline-code-end]


[inline-code-attrs-start title = 'FastComments React Native via Yarn'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
yarn add fastcomments-react-native
[inline-code-end]

The configuration is specified slightly differently compared to the `fastcomments-react` library:

[inline-code-attrs-start title = 'React Native Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import { FastCommentsCommentWidget } from 'fastcomments-react-native';

// ...

  const myTenantId = 'demo'; // Your tenant id. Can be fetched from https://fastcomments.com/auth/my-account/api-secret
  const myAppPageId = 'native-test'; // the ID or URL of the comment thread in your app.
  const config = {
    tenantId: myTenantId,
    urlId: myAppPageId
  };

  return (
      <FastCommentsCommentWidget config={config}/>
  );
[inline-code-end]

You can find the configuration the React Native component supports <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">here</a>.
