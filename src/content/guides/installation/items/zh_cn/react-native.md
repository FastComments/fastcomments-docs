您可以在NPM的<a href="https://www.npmjs.com/package/fastcomments-react-native" target="_blank">这里</a>找到我们的React Native库。

FastComments React Native评论小部件支持与VanillaJS版本相同的所有功能——实时评论、SSO等。

[inline-code-attrs-start title = 'FastComments React Native（通过NPM）'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install --save fastcomments-react-native
[inline-code-end]


[inline-code-attrs-start title = 'FastComments React Native（通过Yarn）'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
yarn add fastcomments-react-native
[inline-code-end]

配置的指定方式与`fastcomments-react`库略有不同：

[inline-code-attrs-start title = 'React Native示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import { FastCommentsCommentWidget } from 'fastcomments-react-native';

// ...

  const tenantId = 'demo'; // 您的租户ID。可以从https://fastcomments.com/auth/my-account/api-secret获取
  const pageId = 'native-test'; // 您应用中评论线程的ID或URL
  const config = {
    tenantId: tenantId,
    urlId: pageId
  };

  return (
      <FastCommentsCommentWidget config={config}/>
  );
[inline-code-end]

如果您在欧盟，您需要设置`region`参数：

[inline-code-attrs-start title = 'React Native - 欧盟'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
  // ...
  const config = {
    tenantId: tenantId,
    urlId: pageId,
    region: 'eu'
  };
  // ...
[inline-code-end]

您可以在<a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">这里</a>找到React Native组件支持的配置。
