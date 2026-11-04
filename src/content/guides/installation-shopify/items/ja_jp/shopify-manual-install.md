[Shopify App Storeアプリ](https://apps.shopify.com/fastcomments)をインストールできない場合でも、テーマを編集してFastCommentsを追加できます。この方法は、すでに所有しているFastCommentsテナントを接続したい場合や、アプリを利用できないShopifyストアフロントに埋め込む場合に便利です。

アプリベースのインストールは、ほとんどのストアで推奨される方法です。アプリが適合しない場合にのみ、この方法を使用してください。

### ステップ1: Shopifyのネイティブコメントを無効にする

Shopify管理画面で、**ブログ記事 > ブログを管理**に移動し、各ブログを開いて、右側のパネルで**コメントを無効にする**を設定します。保存します。

これにより、Shopifyの組み込みコメント機能がFastCommentsと一緒に表示されなくなります。

### ステップ2: ブログテーマテンプレートを開く

Shopify管理画面で:

1. **オンラインストア > テーマ**に移動します。
2. 現在のテーマの下で、**アクション > コードを編集**をクリックします。
3. 左側のファイルブラウザで**Sections**を開き、`main-article.liquid`をクリックします。

これは、Shopifyが単一のブログ記事をレンダリングするために使用するテンプレートです。

### ステップ3: FastCommentsスニペットを貼り付ける

`main-article.liquid`の約100行目までスクロールし、記事本文の閉じタグ`</div>`の直後に移動します。以下のスニペットを貼り付けてください:

[inline-code-attrs-start title = 'Shopify FastCommentsスニペット'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget" class="page-width page-width--narrow"></div>
<script>
    window.fcConfigs = [{
        target: '#fastcomments-widget',
        tenantId: "demo",
        urlId: window.location.pathname
    }];
</script>
[inline-code-end]

`"demo"`を、[fastcomments.com/auth/my-account/api-secret](https://fastcomments.com/auth/my-account/api-secret)から取得した自分のテナントIDに置き換えてください。**保存**をクリックします。

### ステップ4: ショップドメインを承認する

ライブストア上のブログ投稿を開きます。コメントウィジェットの代わりに認証エラーが表示される場合は、FastCommentsにそのストアがこのテナントの使用を許可されていることを伝える必要があります。[ドメインエラー](/guide-installation-shopify.html#shopify-domain-errors)を参照してください。

### 他のページへのFastCommentsの追加

同じスニペットは、商品ページ、カスタムページ、ホームページなど、任意のLiquidテンプレートで機能します。コメントを表示したい場所に貼り付け、ページごとに固定の識別子を使用したい場合は`urlId`を調整してください(例: 商品テンプレートでは`urlId: "{{ product.id }}"`)。