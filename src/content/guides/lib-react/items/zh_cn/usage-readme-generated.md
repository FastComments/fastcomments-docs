### 主 Widget 组件

FastCommentsCommentWidget 组件包含实时的 FastComments 评论小部件。

将下面的 "demo" 替换为您的 "tenantId" — 在 FastComments 管理区域的 [此处](https://fastcomments.com/auth/my-account/api) 可用。

该组件支持许多选项 - 请参阅 src/index.tsx 中的 FastCommentsCommentWidgetConfig。

```tsx
import React, { Component } from 'react'

import {FastCommentsCommentWidget} from 'fastcomments-react'

class Example extends Component {
  render() {
    return <FastCommentsCommentWidget tenantId="demo" />
  }
}
```

### 更新当前页面（针对 SPA）
要更新评论线程所关联的页面/文章，您必须更新配置参数 "urlId" 和 "url"。示例和说明见 [此处](https://github.com/FastComments/fastcomments-react/blob/master/examples/example-paginated/src/PaginatedApp.tsx)。

### 账户区域（注意：欧盟客户）

如果您位于欧盟，您需要告知客户端小部件您的区域。参见 [examples/example-eu](https://github.com/FastComments/fastcomments-react/blob/master/examples/example-eu/src/App.tsx);
否则，您无需定义 `region`。

### 评论计数 Widget

FastCommentsCommentCountWidget 组件包含实时的 FastComments 评论计数小部件。

将下面的 "demo" 替换为您的 "tenantId" — 在 FastComments 管理区域的 [此处](https://fastcomments.com/auth/my-account/api) 可用。

有关支持的配置选项，请参阅 src/index.tsx 中的 FastCommentsCommentCountConfig。

```tsx
import React, { Component } from 'react'

import {FastCommentsCommentCountWidget} from 'fastcomments-react'

class Example extends Component {
  render() {
    return <FastCommentsCommentCountWidget tenantId="demo" urlId="https://example.com/some-page-or-id" />
  }
}
```

### 原生

有关完全原生实现 FastComments，请参阅 [fastcomments-react-native-sdk](https://github.com/FastComments/fastcomments-react-native-sdk)。

对于使用 webview 的 React Native 包装器，请参阅 [fastcomments-react-native](https://github.com/FastComments/fastcomments-react-native).