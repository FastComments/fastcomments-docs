---
The FastComments Rust SDK はいくつかのモジュールで構成されています:

- **Client Module** - FastComments の REST API 向けに自動生成された API クライアント
  - すべての API モデルに対する完全な型定義
  - 認証済み (`DefaultApi`) と公開 (`PublicApi`) の両方のエンドポイント
  - tokio を使用した完全な async/await サポート
  - 詳細な API ドキュメントは [client/README.md](https://github.com/FastComments/fastcomments-rust/blob/main/client/README.md) を参照してください

- **SSO Module** - サーバー側のシングルサインオン（SSO）ユーティリティ
  - ユーザー認証のための安全なトークン生成
  - シンプルとセキュアの両方の SSO モードをサポート
  - HMAC-SHA256 に基づくトークン署名

- **Core Types** - 共有の型定義とユーティリティ
  - コメントモデルとメタデータ構造
  - ユーザーおよびテナントの設定
  - 共通操作のためのヘルパー関数
---