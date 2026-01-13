### PyPI

```bash
pip install fastcomments
```

### ライブラリの内容

このライブラリには2つのモジュールが含まれています: 生成された API クライアントと、手作業で書かれたユーティリティを含むコア Python ライブラリで、SSO サポートを含め API の操作を簡単にします。

- [API クライアントライブラリのドキュメント](https://github.com/FastComments/fastcomments-python/blob/main/client/README.md)
- [コアライブラリのドキュメント（SSO の例を含む）](https://github.com/FastComments/fastcomments-python/blob/main/sso/README.md)

### 公開 API と保護された API

API クライアントには、`DefaultApi` と `PublicApi` の 2 つのクラスがあります。`DefaultApi` は API キーが必要なメソッドを含み、`PublicApi` はブラウザやモバイル端末などから認証なしで直接呼び出せる API コールを含みます。