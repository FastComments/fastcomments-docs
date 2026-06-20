### Nimble の使用

```bash
nimble install fastcomments
```

### ソースからビルド

```bash
nimble build
```

### ライブラリの内容

このライブラリには、生成された API クライアントと、API の操作を容易にする SSO ユーティリティが含まれています。

- [API クライアント ライブラリのドキュメント](https://github.com/FastComments/fastcomments-nim/blob/master/client/README.md)

### パブリック vs 保護された API

API クライアントには、`api_default`、`api_public`、`api_moderation` の 3 つの API モジュールがあります。`api_default` は API キーを必要とするメソッドを含み、`api_public` はブラウザ/モバイルデバイス等から認証なしに直接呼び出せる API コールを含みます。`api_moderation` モジュールはモデレーターダッシュボード用のメソッドを含みます。

`api_moderation` のメソッドは、コメントおよびそのログの一覧取得、カウント、検索、エクスポート；コメントの削除/復元、フラグ付け、レビュー/スパム/承認ステータスの設定、投票の調整、スレッドの再開/クローズなどのモデレーション操作；バン（コメントからのユーザーのバン、バンの解除、事前バンのサマリー、バン状況と設定、バンされたユーザーの数）；およびバッジと信頼（バッジの付与/削除、手動バッジの一覧取得、ユーザーの信頼度の取得/設定、ユーザーの内部プロファイルの取得）をカバーします。すべての `api_moderation` メソッドは `sso` パラメータを受け取り、その呼び出しは SSO モデレーターとして認証されます。