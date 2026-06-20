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

このライブラリには生成された API クライアントと、API の利用を容易にする SSO ユーティリティが含まれます。

- [API クライアントライブラリのドキュメント](https://github.com/FastComments/fastcomments-cpp/blob/master/client/README.md)

### 公開 API と保護された API

API クライアントには、`DefaultApi`、`PublicApi`、および `ModerationApi` の 3 つのクラスがあります。  
`DefaultApi` は API キーを必要とするメソッドを含み、`PublicApi` は
認証なしでブラウザ／モバイル端末等から直接実行できるメソッドを含みます。`ModerationApi` はモデレーターダッシュボードを動かすメソッドを含みます - 一覧表示、
カウント、検索、エクスポート、コメントのログ取得、モデレーション操作（削除/復元、フラグ、レビュー/スパム/承認ステータスの設定、投票の調整、スレッドの再開/クローズ）、
バン（コメントからのバン、バンの取り消し、事前バンの概要、バンのステータスと設定、バンされたユーザーの数）、およびバッジと信頼（バッジの付与/削除、手動バッジ、信頼係数の取得/設定、ユーザーの内部プロファイル）。  
すべての `ModerationApi` メソッドは `sso` パラメータを受け取り、呼び出しは SSO 認証されたモデレーターの代理として実行されます。