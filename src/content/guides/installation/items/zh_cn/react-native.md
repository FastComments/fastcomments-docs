要向您的 React Native 应用添加评论，您可以在 NPM 上找到我们的 React Native 库 <a href="https://www.npmjs.com/package/fastcomments-react-native" target="_blank">这里</a>。

FastComments React Native 评论小部件支持与 VanillaJS 相同的所有功能 — 实时评论、单点登录（SSO）等。

[inline-code-attrs-start title = '通过 NPM 安装 FastComments React Native'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install --save fastcomments-react-native
[inline-code-end]


[inline-code-attrs-start title = '通过 Yarn 安装 FastComments React Native'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
yarn add fastcomments-react-native
[inline-code-end]

配置的指定方式与 `fastcomments-react` 库略有不同：

[inline-code-attrs-start title = 'React Native 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import { FastCommentsCommentWidget } from 'fastcomments-react-native';

// ...

  const tenantId = 'demo'; // 您的租户 ID。可以从 https://fastcomments.com/auth/my-account/api-secret 获取
  const pageId = 'native-test'; // 您应用中评论线程的 ID 或 URL。
  const config = {
    tenantId: tenantId,
    urlId: pageId
  };

  return (
      <FastCommentsCommentWidget config={config}/>
  );
[inline-code-end]

如果您位于欧盟，您需要设置 `region` 参数：

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

您可以在 <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">这里</a> 找到 React Native 组件支持的配置。