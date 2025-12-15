React NativeライブラリはNPMの<a href="https://www.npmjs.com/package/fastcomments-react-native" target="_blank">こちら</a>で見つけることができます。

FastComments React Nativeコメントウィジェットは、VanillaJS版と同じすべての機能（ライブコメント、SSO など）をサポートしています。

[inline-code-attrs-start title = 'FastComments React Native（NPM経由）'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install --save fastcomments-react-native
[inline-code-end]


[inline-code-attrs-start title = 'FastComments React Native（Yarn経由）'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
yarn add fastcomments-react-native
[inline-code-end]

設定は`fastcomments-react`ライブラリとは少し異なる方法で指定されます：

[inline-code-attrs-start title = 'React Nativeの例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import { FastCommentsCommentWidget } from 'fastcomments-react-native';

// ...

  const tenantId = 'demo'; // あなたのテナントID。https://fastcomments.com/auth/my-account/api-secretから取得できます
  const pageId = 'native-test'; // アプリ内のコメントスレッドのIDまたはURL
  const config = {
    tenantId: tenantId,
    urlId: pageId
  };

  return (
      <FastCommentsCommentWidget config={config}/>
  );
[inline-code-end]

EUにいる場合は、`region`パラメータを設定する必要があります：

[inline-code-attrs-start title = 'React Native - EU'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
  // ...
  const config = {
    tenantId: tenantId,
    urlId: pageId,
    region: 'eu'
  };
  // ...
[inline-code-end]

React Nativeコンポーネントがサポートする設定は<a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">こちら</a>で見つけることができます。
