### 메인 위젯 컴포넌트

The FastCommentsCommentWidget 컴포넌트에는 라이브 FastComments 댓글 위젯이 포함되어 있습니다.

아래의 "demo"를 FastComments 관리자 영역의 [여기](https://fastcomments.com/auth/my-account/api)에서 확인 가능한 당신의 "tenantId"로 바꾸세요.

위젯은 많은 옵션을 지원합니다 - 지원되는 설정은 src/index.tsx의 FastCommentsCommentWidgetConfig를 참조하세요.

```tsx
import React, { Component } from 'react'

import {FastCommentsCommentWidget} from 'fastcomments-react'

class Example extends Component {
  render() {
    return <FastCommentsCommentWidget tenantId="demo" />
  }
}
```

### 현재 페이지 업데이트 (SPA용)
댓글 스레드가 연결된 페이지/기사 정보를 업데이트하려면 구성 매개변수 "urlId"와 "url"을 업데이트해야 합니다.
예제와 설명은 [여기](https://github.com/FastComments/fastcomments-react/blob/master/examples/example-paginated/src/PaginatedApp.tsx)를 참조하세요.

### 계정 지역 (주의: EU 고객)

EU에 계신 경우, 클라이언트 위젯에 어느 지역에 있는지 알려주어야 합니다. [examples/example-eu](https://github.com/FastComments/fastcomments-react/blob/master/examples/example-eu/src/App.tsx)를 참조하세요;
그렇지 않은 경우 `region`을 정의할 필요가 없습니다.

### 댓글 카운트 위젯

The FastCommentsCommentCountWidget 컴포넌트에는 라이브 FastComments 댓글 수 위젯이 포함되어 있습니다.

아래의 "demo"를 FastComments 관리자 영역의 [여기](https://fastcomments.com/auth/my-account/api)에서 확인 가능한 당신의 "tenantId"로 바꾸세요.

지원되는 구성 옵션은 src/index.tsx의 FastCommentsCommentCountConfig를 참조하세요.

```tsx
import React, { Component } from 'react'

import {FastCommentsCommentCountWidget} from 'fastcomments-react'

class Example extends Component {
  render() {
    return <FastCommentsCommentCountWidget tenantId="demo" urlId="https://example.com/some-page-or-id" />
  }
}
```

### 네이티브

FastComments를 완전히 네이티브로 구현하려면 [fastcomments-react-native-sdk](https://github.com/FastComments/fastcomments-react-native-sdk)를 참조하세요.

웹뷰를 사용하는 이 라이브러리의 React Native 래퍼는 [fastcomments-react-native](https://github.com/FastComments/fastcomments-react-native)를 참조하세요.