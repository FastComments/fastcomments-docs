---
The **FastComments - Comment Count** ブロックは、単一ページの小さなコメント件数を表示します。ブログ記事一覧や商品カード、コメント付きページへのリンクを含む任意のテンプレートで使用して、訪問者がクリックする前に各スレッドの活発さを確認できるようにします。

### ブロックを追加する

1. Shopifyテーマエディタを開きます。
2. カウントを表示したいテンプレートを開きます。例: **Blog** テンプレート（投稿一覧）や商品一覧セクション。
3. 各アイテムをレンダリングするセクションで **ブロックを追加** をクリックします。
4. 「**アプリ**」の下で **FastComments - Comment Count** を選択します。
5. 「**保存**」をクリックします。

### 設定

| Setting | What it does | Default |
|---|---|---|
| Tenant ID (optional) | カウントが読み取る FastComments テナントを上書きします。ストアの自動構成されたテナントを使用する場合は空欄のままにします。 | (空欄) |
| Custom URL ID | カウントが参照するページ識別子を上書きします。カウントが追跡する FastComments ブロックとは別のページにカウントがある場合に使用します。 | (自動検出) |

### カウントがコメントスレッドと一致する仕組み

Comment Count ブロックは **FastComments** ブロックと同じ自動検出ロジックを使用します:

- Blog post template: `shopify-article-{article.id}`
- Product template: `shopify-product-{product.id}`
- Other templates: the request path

ページ上の **FastComments** ブロックに **Custom URL ID** を設定している場合は、同じ Custom URL ID を Comment Count ブロックにも設定して、同じスレッドを指すようにしてください。

### ヒント

- ページ上のすべてのアイテムのカウントは1回のリクエストで取得されるため、長いリストの各アイテムにブロックを追加しても追加の往復コストは発生しません。
- リスト内の各記事や商品につき1つの Comment Count ブロックを配置するのが想定される使い方です。必要に応じてブロックを何度でも追加できます。

---