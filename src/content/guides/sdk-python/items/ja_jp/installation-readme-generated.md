### GitHub からインストール

リリースタグから直接インストールします（推奨、完全に再現可能）：

```bash
pip install git+https://github.com/fastcomments/fastcomments-python.git@v3.1.0
```

ブランチではなくタグを固定することで、ビルドが決定的になります。同じ形式は `requirements.txt` でも機能します：

```
fastcomments @ git+https://github.com/fastcomments/fastcomments-python.git@v3.1.0
```

タグ付きの [GitHub Release](https://github.com/FastComments/fastcomments-python/releases) には、直接バイナリアーティファクトをインストールしたい場合に使用できるビルド済みの wheel が添付されています。

### ライブラリの内容

このライブラリには、生成された API クライアントと、API の利用を容易にする手書きユーティリティ（SSO サポートを含む）を含むコア Python ライブラリの 2 つのモジュールが含まれています。

- [API Client Library Docs](https://github.com/FastComments/fastcomments-python/blob/main/client/README.md)
- [Core Library Docs, Including SSO Examples](https://github.com/FastComments/fastcomments-python/blob/main/sso/README.md)

### 公開 API と保護された API

API クライアントには、`DefaultApi`、`PublicApi`、`ModerationApi` の 3 つのクラスがあります。`DefaultApi` には API キーが必要なメソッドが含まれ、`PublicApi` には認証なしでブラウザやモバイルデバイスなどから直接呼び出せるメソッドが含まれます。`ModerationApi` は、ライブかつ高速なモデレーション API の豊富なスイートを提供します。すべての `ModerationApi` メソッドは `sso` パラメータを受け取り、SSO または FastComments.com のセッションクッキーで認証できます。