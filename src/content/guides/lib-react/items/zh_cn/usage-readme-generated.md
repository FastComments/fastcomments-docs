### 主要 Widget 组件

FastCommentsCommentWidget 组件包含实时的 FastComments 评论小部件。

将下方的 "demo" 替换为您的 "tenantId" — 可在 FastComments 管理后台的 [此处](https://fastcomments.com/auth/my-account/api) 获取。

该小部件支持许多选项——参见 src/index.tsx 中的 FastCommentsCommentWidgetConfig。

```tsx
import React, { Component } from 'react'

import {FastCommentsCommentWidget} from 'fastcomments-react'

class Example extends Component {
  render() {
    return <FastCommentsCommentWidget tenantId="demo" />
  }
}
```

### 更新当前页面（适用于单页应用 SPA）
要更新评论线程所绑定的页面/文章，必须更新配置参数 "urlId" 和 "url"。
详见示例与说明 [此处](https://github.com/FastComments/fastcomments-react/blob/master/examples/example-paginated/src/PaginatedApp.tsx)。

### 帐户区域（注意：欧盟客户）

如果您位于欧盟，您需要告知客户端小部件您的所在区域。参见 [examples/example-eu](/examples/example-eu/src/App.tsx);
否则，您无需定义 `region`。

### 评论计数小部件

FastCommentsCommentCountWidget 组件包含实时的 FastComments 评论计数小部件。

将下方的 "demo" 替换为您的 "tenantId" — 可在 FastComments 管理后台的 [此处](https://fastcomments.com/auth/my-account/api) 获取。

支持的配置选项请参见 src/index.tsx 中的 FastCommentsCommentCountConfig。

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

要实现完全原生的 FastComments，请参见 [fastcomments-react-native-sdk](https://github.com/FastComments/fastcomments-react-native-sdk)。

若要使用 webview 的 React Native 包装器，请参见 [fastcomments-react-native](https://github.com/FastComments/fastcomments-react-native).