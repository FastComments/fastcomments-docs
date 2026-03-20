### メインウィジェットコンポーネント

The FastCommentsCommentWidget component contains the live FastComments comment widget.

下の "demo" をあなたの "tenantId" に置き換えてください - FastComments 管理画面の [こちら](https://fastcomments.com/auth/my-account/api) で確認できます。

このウィジェットは多くのオプションをサポートしています - 詳細は src/index.tsx の FastCommentsCommentWidgetConfig を参照してください。

```tsx
import React, { Component } from 'react'

import {FastCommentsCommentWidget} from 'fastcomments-react'

class Example extends Component {
  render() {
    return <FastCommentsCommentWidget tenantId="demo" />
  }
}
```

### 現在のページの更新（SPAの場合）
コメントスレッドが紐付いているページ/記事を更新するには、設定パラメータの "urlId" と "url" を更新する必要があります。
例と説明は [こちら](https://github.com/FastComments/fastcomments-react/blob/master/examples/example-paginated/src/PaginatedApp.tsx) を参照してください。

### アカウントのリージョン（注意：EUのお客様）

EU にいる場合、クライアントウィジェットにリージョンを指定する必要があります。See [examples/example-eu](https://github.com/FastComments/fastcomments-react/blob/master/examples/example-eu/src/App.tsx);
それ以外の場合、`region` を定義する必要はありません。

### コメントカウントウィジェット

The FastCommentsCommentCountWidget component contains the live FastComments comment count widget.

下の "demo" をあなたの "tenantId" に置き換えてください - FastComments 管理画面の [こちら](https://fastcomments.com/auth/my-account/api) で確認できます。

サポートされている設定オプションは src/index.tsx の FastCommentsCommentCountConfig を参照してください。

```tsx
import React, { Component } from 'react'

import {FastCommentsCommentCountWidget} from 'fastcomments-react'

class Example extends Component {
  render() {
    return <FastCommentsCommentCountWidget tenantId="demo" urlId="https://example.com/some-page-or-id" />
  }
}
```

### ネイティブ

FastComments を完全にネイティブで実装する場合は、[fastcomments-react-native-sdk](https://github.com/FastComments/fastcomments-react-native-sdk) を参照してください。

このライブラリの React Native ラッパー（webview を使用）については、[fastcomments-react-native](https://github.com/FastComments/fastcomments-react-native) を参照してください。