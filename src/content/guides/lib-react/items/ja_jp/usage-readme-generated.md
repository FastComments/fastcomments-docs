### The Main Widget Component

The FastCommentsCommentWidget component はライブの FastComments コメントウィジェットを含みます。

下の "demo" をあなたの "tenantId" に置き換えてください - FastComments 管理画面の [ここ](https://fastcomments.com/auth/my-account/api) で入手できます。

ウィジェットはいくつものオプションをサポートしています - 詳しくは src/index.tsx の FastCommentsCommentWidgetConfig を参照してください。

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
コメントスレッドが紐づいているページ／記事を更新するには、設定パラメーター "urlId" と "url" を更新する必要があります。
例と説明は [こちら](https://github.com/FastComments/fastcomments-react/blob/master/examples/example-paginated/src/PaginatedApp.tsx) を参照してください。

### Account Region (ATTENTION: EU Customers)

EU にいる場合、クライアントウィジェットにあなたのリージョンを伝える必要があります。 [examples/example-eu](/examples/example-eu/src/App.tsx) を参照してください;
それ以外の場合は `region` を定義する必要はありません。

### The Comment Count Widget

The FastCommentsCommentCountWidget component はライブの FastComments コメント数ウィジェットを含みます。

下の "demo" をあなたの "tenantId" に置き換えてください - FastComments 管理画面の [ここ](https://fastcomments.com/auth/my-account/api) で入手できます。

サポートされている設定オプションについては src/index.tsx の FastCommentsCommentCountConfig を参照してください。

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

FastComments を完全にネイティブで実装する方法については [fastcomments-react-native-sdk](https://github.com/FastComments/fastcomments-react-native-sdk) を参照してください。

このライブラリの React Native ラッパー（WebView を使用）については [fastcomments-react-native](https://github.com/FastComments/fastcomments-react-native) を参照してください。