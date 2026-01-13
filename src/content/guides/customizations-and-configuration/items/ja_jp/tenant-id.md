[related-parameter-start name = 'tenantId'; type = 'string'; related-parameter-end]

コメントウィジェットは例えばテナントIDに "demo" を使って利用できることに気付くかもしれません:

[code-example-start config = {tenantId: 'demo'}; linesToHighlight = [5]; title = 'Demo Tenant ID'; code-example-end]

これはコメントウィジェットを試して遊ぶためのものです。 本番環境では、次のようにあなたのテナントIDを渡します:

[code-example-start config = {tenantId: '{{{ExampleTenantId}}}'}; linesToHighlight = [5]; title = 'Defining Your Tenant ID'; code-example-end]

あなたのテナントIDは、コメントウィジェットの<a href="https://fastcomments.com/auth/my-account/get-acct-code" target="_blank">あなたのアカウント内のコードスニペット</a>に既に適用されているのが確認できます。

テナントIDを見つけたり、APIキーを管理したりするには、[API認証情報ページ](https://fastcomments.com/auth/my-account/api-secret)をご覧ください。

以降、FastCommentsにログインしている場合、コード例ではあなたの実際のテナントIDが使用されます（https://fastcomments.com にログインしている場合）。