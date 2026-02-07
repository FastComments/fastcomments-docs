React Native アプリにコメントを追加するには、NPM 上の当社の React Native ライブラリを<a href="https://www.npmjs.com/package/fastcomments-react-native" target="_blank">こちら</a>で確認できます。

FastComments の React Native コメントウィジェットは、VanillaJS のウィジェットと同様の機能（ライブコメント、sso など）をすべてサポートします。

[inline-code-attrs-start title = 'FastComments React Native（NPM経由）'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install --save fastcomments-react-native
[inline-code-end]


[inline-code-attrs-start title = 'FastComments React Native（Yarn経由）'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
yarn add fastcomments-react-native
[inline-code-end]

設定は `fastcomments-react` ライブラリと比べて少し異なる方法で指定します:

[inline-code-attrs-start title = 'React Native の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import { FastCommentsCommentWidget } from 'fastcomments-react-native';

// ...

  const tenantId = 'demo'; // あなたのテナント ID。https://fastcomments.com/auth/my-account/api-secret から取得できます
  const pageId = 'native-test'; // アプリ内のコメントスレッドの ID または URL。
  const config = {
    tenantId: tenantId,
    urlId: pageId
  };

  return (
      <FastCommentsCommentWidget config={config}/>
  );
[inline-code-end]

EU にいる場合は、`region` パラメータを設定してください:

[inline-code-attrs-start title = 'React Native - EU（EU向け）'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
  // ...
  const config = {
    tenantId: tenantId,
    urlId: pageId,
    region: 'eu'
  };
  // ...
[inline-code-end]

React Native コンポーネントがサポートする設定は <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">こちら</a>で確認できます。