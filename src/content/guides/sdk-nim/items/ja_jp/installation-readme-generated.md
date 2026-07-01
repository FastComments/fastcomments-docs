### Nimble の使用

```bash
nimble install fastcomments
```

### ソースからビルド

```bash
nimble build
```

### ライブラリの内容

このライブラリには、生成された API クライアントと、API の使用を容易にする SSO ユーティリティが含まれています。

- [API Client Library Docs](https://github.com/FastComments/fastcomments-nim/blob/master/client/README.md)

### パブリック API とセキュア API

API クライアントには、`api_default`、`api_public`、`api_moderation` の 3 つの API モジュールがあります。`api_default` には API キーが必要なメソッドが含まれ、`api_public` には認証なしでブラウザやモバイルデバイス等から直接実行できる API 呼び出し  
が含まれます。`api_moderation` モジュールにはモデレーターダッシュボード用のメソッドが含まれています。

`api_moderation` モジュールは、ライブおよび高速モデレーション API の幅広いスイートを提供します。すべての `api_moderation` メソッドは `sso` パラメータを受け取り、SSO または FastComments.com のセッションクッキーで認証できます。