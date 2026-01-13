### Nimble の使用方法

```bash
nimble install fastcomments
```

### ソースからビルドする

```bash
nimble build
```

### ライブラリの内容

このライブラリには、生成された API クライアントと、API の操作を簡単にする SSO ユーティリティが含まれています。

- [API クライアントライブラリのドキュメント](https://github.com/FastComments/fastcomments-nim/blob/master/client/README.md)

### 公開 API と保護された API

API クライアントには、`api_default` と `api_public` の 2 つの API モジュールがあります。`api_default` は API キーを必要とするメソッドを含み、`api_public` は認証なしでブラウザやモバイル端末などから直接実行できる API 呼び出しを含みます。