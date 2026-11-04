### メインウィジェットコンポーネント

FastCommentsCommentWidget コンポーネントには、ライブの FastComments コメントウィジェットが含まれています。

以下の "demo" をあなたの "tenantId" に置き換えてください。tenantId は FastComments 管理画面の[こちら](https://fastcomments.com/auth/my-account/api)から確認できます。

ウィジェットは多くのオプションをサポートしています。詳細は src/index.tsx の FastCommentsCommentWidgetConfig を参照してください。

```tsx
import React, { Component } from 'react'

import {FastCommentsCommentWidget} from 'fastcomments-react'

class Example extends Component {
  render() {
    return <FastCommentsCommentWidget tenantId="demo" />
  }
}
```

### 現在のページの更新(SPAの場合)
コメントスレッドが紐付いているページ/記事を更新するには、設定パラメータの "urlId" と "url" を更新する必要があります。
例と説明については[こちら](https://github.com/FastComments/fastcomments-react/blob/master/examples/example-paginated/src/PaginatedApp.tsx)を参照してください。

### アカウントのリージョン(注意: EUのお客様へ)

EU にお住まいの場合、クライアントウィジェットにリージョンを指定する必要があります。[examples/example-eu](https://github.com/FastComments/fastcomments-react/blob/master/examples/example-eu/src/App.tsx) を参照してください。
それ以外の場合は、`region` を定義する必要はありません。

### コメント数ウィジェット

FastCommentsCommentCountWidget コンポーネントには、ライブの FastComments コメント数ウィジェットが含まれています。

以下の "demo" をあなたの "tenantId" に置き換えてください。tenantId は FastComments 管理画面の[こちら](https://fastcomments.com/auth/my-account/api)から確認できます。

サポートされている設定オプションについては、src/index.tsx の FastCommentsCommentCountConfig を参照してください。

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

FastComments の完全にネイティブな実装については、[fastcomments-react-native-sdk](https://github.com/FastComments/fastcomments-react-native-sdk) を参照してください。

このライブラリの webview を使用した React Native ラッパーについては、[fastcomments-react-native](https://github.com/FastComments/fastcomments-react-native) を参照してください。