### Install from GitHub

リリースタグから直接インストールします（推奨、完全に再現可能）:

```bash
pip install git+https://github.com/fastcomments/fastcomments-python.git@v3.0.0
```

ビルドが決定的になるよう、ブランチではなくタグを固定してください。同じ形式は `requirements.txt` でも機能します:

```
fastcomments @ git+https://github.com/fastcomments/fastcomments-python.git@v3.0.0
```

各タグ付けされた [GitHub Release](https://github.com/fastcomments/fastcomments-python/releases) には、バイナリ成果物として直接インストールできるビルトホイールも添付されています。

### Library Contents

このライブラリは 2 つのモジュールを含みます: 生成された API クライアントと、API の利用を容易にする手書きユーティリティを含むコア Python ライブラリ（SSO サポートを含む）。

- [API Client Library Docs](https://github.com/FastComments/fastcomments-python/blob/main/client/README.md)
- [Core Library Docs, Including SSO Examples](https://github.com/FastComments/fastcomments-python/blob/main/sso/README.md)

### Public vs Secured APIs

API クライアントには `DefaultApi`、`PublicApi`、`ModerationApi` の 3 クラスがあります。`DefaultApi` には API キーが必要なメソッドが含まれ、`PublicApi` には認証なしでブラウザ/モバイルデバイス等から直接呼び出せるメソッドが含まれます。`ModerationApi` は、ライブかつ高速なモデレーション API の包括的なスイートを提供します。すべての `ModerationApi` メソッドは `sso` パラメータを受け取り、SSO または FastComments.com のセッションクッキーで認証できます。