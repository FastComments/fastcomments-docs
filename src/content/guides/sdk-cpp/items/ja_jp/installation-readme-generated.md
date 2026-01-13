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

このライブラリには、生成された API クライアントと、API の利用を簡易にする SSO ユーティリティが含まれています。

- [API クライアント ライブラリのドキュメント](https://github.com/FastComments/fastcomments-cpp/blob/master/client/README.md)

### 公開 API と保護された API

API クライアントには、`DefaultAPI` と `PublicAPI` の2つのクラスがあります。`DefaultAPI` はあなたの API キーを必要とするメソッドを含み、`PublicAPI` はブラウザやモバイル端末などから認証なしで直接実行できる API 呼び出しを含みます。