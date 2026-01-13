您可以在NPM的<a href="https://www.npmjs.com/package/fastcomments-react-native" target="_blank">這裡</a>找到我們的React Native函式庫。

FastComments React Native評論小工具支援與VanillaJS版本相同的所有功能——即時評論、SSO等。

[inline-code-attrs-start title = 'FastComments React Native（透過NPM）'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install --save fastcomments-react-native
[inline-code-end]


[inline-code-attrs-start title = 'FastComments React Native（透過Yarn）'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
yarn add fastcomments-react-native
[inline-code-end]

配置的指定方式與`fastcomments-react`函式庫略有不同：

[inline-code-attrs-start title = 'React Native範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import { FastCommentsCommentWidget } from 'fastcomments-react-native';

// ...

  const tenantId = 'demo'; // 您的租戶ID。可以從https://fastcomments.com/auth/my-account/api-secret獲取
  const pageId = 'native-test'; // 您應用中評論串的ID或URL
  const config = {
    tenantId: tenantId,
    urlId: pageId
  };

  return (
      <FastCommentsCommentWidget config={config}/>
  );
[inline-code-end]

如果您在歐盟，您需要設定`region`參數：

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

您可以在<a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">這裡</a>找到React Native元件支援的配置。
