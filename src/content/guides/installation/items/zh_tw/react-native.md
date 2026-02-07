要在您的 React Native 應用中新增留言，您可以在 NPM 上找到我們的 React Native 函式庫 <a href="https://www.npmjs.com/package/fastcomments-react-native" target="_blank">這裡</a>。

FastComments React Native 留言元件支援與 VanillaJS 相同的所有功能 — 即時留言、SSO 等等。

[inline-code-attrs-start title = 'FastComments React Native（透過 NPM）'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install --save fastcomments-react-native
[inline-code-end]


[inline-code-attrs-start title = 'FastComments React Native（透過 Yarn）'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
yarn add fastcomments-react-native
[inline-code-end]

設定方式與 `fastcomments-react` 函式庫相比略有不同：

[inline-code-attrs-start title = 'React Native 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import { FastCommentsCommentWidget } from 'fastcomments-react-native';

// ...

  const tenantId = 'demo'; // 您的 tenant id。可從 https://fastcomments.com/auth/my-account/api-secret 取得
  const pageId = 'native-test'; // 您應用中留言討論串的 ID 或 URL。
  const config = {
    tenantId: tenantId,
    urlId: pageId
  };

  return (
      <FastCommentsCommentWidget config={config}/>
  );
[inline-code-end]

如果您位於歐盟，您會想要設定 `region` 參數：

[inline-code-attrs-start title = 'React Native - 歐盟'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
  // ...
  const config = {
    tenantId: tenantId,
    urlId: pageId,
    region: 'eu'
  };
  // ...
[inline-code-end]

您可以在 <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">這裡</a> 找到 React Native 元件所支援的設定。