The **FastComments - Reviews Summary** ブロックは、ページの集計された星評価とレビューの内訳を表示します。標準的なレビューのレイアウト（上部に概要、その下にレビュー投稿フォームとレビュー）を実現するには、商品テンプレート上の **FastComments** ブロックと組み合わせて使用してください。

### 前提条件: Ratings & Reviews を設定する

Reviews Summary ブロックは、ストア用に構成した評価（rating）およびレビューの質問を表示します。まずそれらを設定してください:

1. Shopify 管理画面で FastComments アプリを開きます。
2. **Ratings & Reviews Helper** タイルをクリックするか、直接 [Ratings & Reviews Helper](https://fastcomments.com/auth/my-account/ratings-reviews-helper?source=shopify) を開きます。
3. 各レビュワーに回答してほしい質問を追加します（総合の星評価、「フィット感はどうだったか」など）。

質問が設定されていないと、サマリーブロックは集計するものがありません。

### ブロックを追加する

1. Shopify テーマエディタを開きます。
2. **Product** テンプレート（またはサマリーを表示したいページのテンプレート）を開きます。
3. ページセクションの上部、**FastComments** ブロックが表示される予定の上あたりで **Add block** をクリックします。
4. **Apps** の下で **FastComments - Reviews Summary** を選択します。
5. まだ追加していない場合は、同じページの下側に **FastComments** ブロックを追加して、訪問者がレビューを投稿できるようにします。
6. **Save** をクリックします。

### 設定

| Setting | What it does | Default |
|---|---|---|
| Tenant ID (optional) | サマリーが読み取る FastComments テナントを上書きします。ストアの自動構成されたテナントを使用する場合は空白のままにしてください。 | (blank) |
| Custom URL ID | サマリーが集計するページ識別子を上書きします。サマリーが反映する FastComments ブロックとは別のページに配置されている場合に使用してください。 | (auto-detected) |

### サマリーがレビューと一致する仕組み

Reviews Summary ブロックは **FastComments** ブロックと同じ自動検出ロジックを使用します:

- Product テンプレート: `shopify-product-{product.id}`
- Blog post テンプレート: `shopify-article-{article.id}`
- その他のテンプレート: リクエストパス

通常の商品ページでは、サマリーとコメントスレッドは自動的に同じ URL ID を共有するため、追加の設定は必要ありません。

### ヒント

- サマリーは読み取り専用です。レビューを収集するには、同一ページに **FastComments** ブロックが必要です。
- Ratings & Reviews Helper で質問を変更した場合、既に収集したレビューに対してもサマリーは新しい質問セットで再計算されます。