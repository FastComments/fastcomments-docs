### The Main Widget Component

FastCommentsCommentWidget 元件包含即時的 FastComments 留言小工具。

將下方的 "demo" 替換為您的 "tenantId" - 可在 FastComments 管理後台的 [這裡](https://fastcomments.com/auth/my-account/api) 取得。

此小工具支援許多選項 - 請參閱 src/index.tsx 中的 FastCommentsCommentWidgetConfig。

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
若要更新留言串所綁定的頁面/文章，您必須更新設定參數 "urlId" 與 "url"。
範例與說明請見 [這裡](https://github.com/FastComments/fastcomments-react/blob/master/examples/example-paginated/src/PaginatedApp.tsx)。

### Account Region (ATTENTION: EU Customers)

如果您位於歐盟，您會想要告知用戶端小工具您所在的區域。請參閱 [examples/example-eu](https://github.com/FastComments/fastcomments-react/blob/master/examples/example-eu/src/App.tsx)；
否則，您不需要定義 `region`。

### The Comment Count Widget

FastCommentsCommentCountWidget 元件包含即時的 FastComments 留言數量小工具。

將下方的 "demo" 替換為您的 "tenantId" - 可在 FastComments 管理後台的 [這裡](https://fastcomments.com/auth/my-account/api) 取得。

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

### Native

如需完全原生實作的 FastComments，請參閱 [fastcomments-react-native-sdk](https://github.com/FastComments/fastcomments-react-native-sdk)。

如需以 webview 包裝此函式庫的 React Native 外層，請參閱 [fastcomments-react-native](https://github.com/FastComments/fastcomments-react-native).