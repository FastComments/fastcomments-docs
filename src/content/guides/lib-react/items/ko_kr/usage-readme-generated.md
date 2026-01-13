### The Main Widget Component

FastCommentsCommentWidget 컴포넌트는 실시간 FastComments 댓글 위젯을 포함합니다.

아래의 "demo"를 FastComments 관리 영역의 [여기](https://fastcomments.com/auth/my-account/api)에 있는 "tenantId"로 바꾸십시오.

이 위젯은 많은 옵션을 지원합니다 - src/index.tsx에 있는 FastCommentsCommentWidgetConfig를 참조하십시오.

```tsx
import React, { Component } from 'react'

import {FastCommentsCommentWidget} from 'fastcomments-react'

class Example extends Component {
  render() {
    return <FastCommentsCommentWidget tenantId="demo" />
  }
}
```

### Updating The Current Page (For SPAs)
댓글 스레드가 연결된 페이지/게시물을 업데이트하려면 구성 파라미터 "urlId"와 "url"을 업데이트해야 합니다.
예제와 설명은 [여기](https://github.com/FastComments/fastcomments-react/blob/master/examples/example-paginated/src/PaginatedApp.tsx)를 참조하세요.

### Account Region (ATTENTION: EU Customers)

EU에 있는 경우, 클라이언트 위젯에 귀하의 지역을 알려야 합니다. [examples/example-eu](/examples/example-eu/src/App.tsx)를 참조하십시오;
그렇지 않으면 `region`을 정의할 필요가 없습니다.

### The Comment Count Widget

FastCommentsCommentCountWidget 컴포넌트는 실시간 FastComments 댓글 수 위젯을 포함합니다.

아래의 "demo"를 FastComments 관리 영역의 [여기](https://fastcomments.com/auth/my-account/api)에 있는 "tenantId"로 바꾸십시오.

지원되는 구성 옵션은 src/index.tsx의 FastCommentsCommentCountConfig를 참조하십시오.

```tsx
import React, { Component } from 'react'

import {FastCommentsCommentCountWidget} from 'fastcomments-react'

class Example extends Component {
  render() {
    return <FastCommentsCommentCountWidget tenantId="demo" urlId="https://example.com/some-page-or-id" />
  }
}
```

### Native

FastComments의 완전한 네이티브 구현은 [fastcomments-react-native-sdk](https://github.com/FastComments/fastcomments-react-native-sdk)를 참조하십시오.

웹뷰를 사용하는 이 라이브러리의 React Native 래퍼는 [fastcomments-react-native](https://github.com/FastComments/fastcomments-react-native)를 참조하십시오.