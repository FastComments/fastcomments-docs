React Native 라이브러리는 NPM <a href="https://www.npmjs.com/package/fastcomments-react-native" target="_blank">여기</a>에서 찾을 수 있습니다.

FastComments React Native 댓글 위젯은 VanillaJS 버전과 동일한 모든 기능(실시간 댓글, SSO 등)을 지원합니다.

[inline-code-attrs-start title = 'FastComments React Native (NPM 사용)'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install --save fastcomments-react-native
[inline-code-end]


[inline-code-attrs-start title = 'FastComments React Native (Yarn 사용)'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
yarn add fastcomments-react-native
[inline-code-end]

구성은 `fastcomments-react` 라이브러리와 약간 다르게 지정됩니다:

[inline-code-attrs-start title = 'React Native 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import { FastCommentsCommentWidget } from 'fastcomments-react-native';

// ...

  const tenantId = 'demo'; // 귀하의 테넌트 ID. https://fastcomments.com/auth/my-account/api-secret에서 가져올 수 있습니다
  const pageId = 'native-test'; // 앱의 댓글 스레드의 ID 또는 URL
  const config = {
    tenantId: tenantId,
    urlId: pageId
  };

  return (
      <FastCommentsCommentWidget config={config}/>
  );
[inline-code-end]

EU에 계신 경우 `region` 매개변수를 설정해야 합니다:

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

React Native 컴포넌트가 지원하는 구성은 <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">여기</a>에서 찾을 수 있습니다.
