FastComments Rust SDK は複数のモジュールで構成されています:

- **Client Module** - FastComments REST API の API クライアント
  - すべての API モデルの完全な型定義
  - FastComments のすべてのメソッドをカバーする 3 つの API クライアント:
    - `default_api` (**DefaultApi**) - サーバーサイドで使用するための API キー認証メソッド
    - `public_api` (**PublicApi**) - ブラウザやモバイルアプリから安全に呼び出せる、API キー不要の公開メソッド
    - `moderation_api` (**ModerationApi**) - ライブかつ高速なモデレーション API の包括的スイート。すべての Moderation メソッドは `sso` パラメータを受け取り、SSO または FastComments.com のセッションクッキーで認証できます。
  - tokio によるフル async/await サポート
  - 詳細な API ドキュメントは [client/README.md](https://github.com/FastComments/fastcomments-rust/blob/main/client/README.md) を参照してください

- **SSO Module** - サーバーサイドのシングルサインオンユーティリティ
  - ユーザー認証のための安全なトークン生成
  - シンプルモードとセキュアモードの両方に対応
  - HMAC-SHA256 に基づくトークン署名

- **Core Types** - 共有型定義とユーティリティ
  - コメントモデルとメタデータ構造
  - ユーザーおよびテナント設定
  - 共通操作のためのヘルパー関数