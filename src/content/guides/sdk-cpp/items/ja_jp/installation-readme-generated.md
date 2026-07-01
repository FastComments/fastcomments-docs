### 依存関係のインストール

```bash
sudo apt install libcpprest-dev libboost-all-dev
```

### ソースからのビルド

```bash
mkdir build
cd build
cmake ..
make
```

### インストール

```bash
sudo make install
```

### ライブラリの内容

このライブラリには、生成された API クライアントと、API の操作を容易にする SSO ユーティリティが含まれています。

- [API クライアント ライブラリ ドキュメント](https://github.com/FastComments/fastcomments-cpp/blob/master/client/README.md)

### 公開 API と保護された API

API クライアントには、`DefaultApi`、`PublicApi`、`ModerationApi` の 3 つのクラスがあります。`DefaultApi` には API キーが必要なメソッドが含まれ、`PublicApi` には
認証なしでブラウザやモバイルデバイス等から直接呼び出せるメソッドが含まれます。`ModerationApi` は、ライブかつ高速な多数のモデレーション API を提供します。すべての `ModerationApi` メソッドは `sso` パラメータを受け取り、SSO または FastComments.com のセッションクッキーで認証できます。