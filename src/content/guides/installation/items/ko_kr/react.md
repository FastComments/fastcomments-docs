React로 구축된 사이트에 댓글을 추가하려면, NPM에서 저희 React 라이브러리를 <a href="https://www.npmjs.com/package/fastcomments-react" target="_blank">여기</a>에서 확인할 수 있습니다.

FastComments React 댓글 위젯은 VanillaJS 버전과 동일한 모든 기능을 지원합니다 - 실시간 댓글, sso, 등.

[inline-code-attrs-start title = 'NPM을 통한 FastComments React'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install --save fastcomments-react
[inline-code-end]


[inline-code-attrs-start title = 'Yarn을 통한 FastComments React'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
yarn add fastcomments-react
[inline-code-end]


[inline-code-attrs-start title = 'React 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import React, { Component } from 'react'

import {FastCommentsCommentWidget} from 'fastcomments-react'

class Example extends Component {
  render() {
    return <FastCommentsCommentWidget tenantId="demo" />
  }
}
[inline-code-end]

EU에 있는 경우, 다음과 같이 `region` 매개변수를 설정하세요:

[inline-code-attrs-start title = 'React 예제 - EU'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
    <FastCommentsCommentWidget tenantId="demo" region="eu" />
[inline-code-end]

React 컴포넌트가 지원하는 구성은 <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">여기</a>에서 확인할 수 있습니다.

---