FastComments は、リクエストがあなたのサイトから来ていることを確認するために、アカウントへのリクエストを認証します。そのため、FastComments をインストールしたいサイト（または複数のサイト）を把握する必要があります。

FastComments は、ドメインおよびサブドメインによる認証をサポートしています。

サイト `https://example.com` を例に取ります。この場合、"`example.com`" がドメインです。`example.com` は `example.com` と `www.example.com` の両方をサポートします。"www" をサブドメインと呼びます。

例:

- `blog.example.com` のみを許可する場合:
  - `blog.example.com` をドメインに追加します。
- `www.example.com`、`somesite.example.com`、および `example.com` を許可する場合:
  - `example.com` をドメインに追加します。
  - これは、アカウントに **1 つのドメイン** が関連付けられているものとして請求されます。
- ワイルドカードサブドメインも追加できます。例: *myname.vercel.app* 
  - これも、アカウントに **1 つのドメイン** が関連付けられているものとして請求されます。

ブログプラットフォームを使用していて、サブドメインが付与されている場合は、アカウントに **サブドメインを含む完全なドメイン** を追加する必要があります。例: `cats.blogger.com`.

`My Domains` ページにアクセスし、下部の `Add a Domain` をクリックしてドメインをアカウントに追加できます:

[app-screenshot-start url='/auth/my-account/configure-domains'; selector = '.content'; alt='アカウント上のドメインを一覧表示し、下部に「Add a Domain」ボタンがある My Domains ページ'; title='My Domains ページ' app-screenshot-end]

トライアル期間中は、**ドメインからのリクエストがあると、ドメインが自動的にアカウントに追加されます**。ただし、この期間が終了した後は、セキュリティ上の理由で明示的に追加する必要があります。この自動的な動作が発生した際には、メールが送信されます。

ローカル開発用に `localhost` を追加する必要は **ありません**。デフォルトで許可されています。

#### API 経由で

ドメインは、[DomainConfigs API を使用して](/guide-api.html#domain-config-structure) 追加および構成することもできます。