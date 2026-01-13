### 主要 Widget 元件

The FastCommentsCommentWidget component contains the live FastComments comment widget.

請將下方的 "demo" 替換為您的 "tenantId" —— 可在 FastComments 管理區的 [這裡](https://fastcomments.com/auth/my-account/api) 取得。

該小工具支援許多選項 —— 請參閱 FastCommentsCommentWidgetConfig 在 src/index.tsx。

```tsx
import React, { Component } from 'react'

import {FastCommentsCommentWidget} from 'fastcomments-react'

class Example extends Component {
  render() {
    return <FastCommentsCommentWidget tenantId="demo" />
  }
}
```

### 更新目前的頁面（適用於 SPA）
若要更新留言串所綁定的頁面/文章，您必須更新設定參數 "urlId" 與 "url"。
請參閱範例與說明 [這裡](https://github.com/FastComments/fastcomments-react/blob/master/examples/example-paginated/src/PaginatedApp.tsx)。

### 帳戶區域（注意：歐盟客戶）

如果您位於歐盟，您需要告知客戶端小工具您所在的區域。請參閱 [examples/example-eu](/examples/example-eu/src/App.tsx);
否則，您不必定義 `region`。

### 留言計數小工具

The FastCommentsCommentCountWidget component contains the live FastComments comment count widget.

請將下方的 "demo" 替換為您的 "tenantId" —— 可在 FastComments 管理區的 [這裡](https://fastcomments.com/auth/my-account/api) 取得。

支援的設定選項請參閱 src/index.tsx 中的 FastCommentsCommentCountConfig。

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

如需完整原生的 FastComments 實作，請參閱 [fastcomments-react-native-sdk](https://github.com/FastComments/fastcomments-react-native-sdk)。

若要在 React Native 中以 webview 包裝此函式庫，請參閱 [fastcomments-react-native](https://github.com/FastComments/fastcomments-react-native).