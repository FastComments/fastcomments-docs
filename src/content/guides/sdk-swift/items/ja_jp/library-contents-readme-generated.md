The FastComments Swift SDK は、いくつかのモジュールで構成されています:

- **Client Module** - FastComments REST API 用に自動生成された API クライアント
  - すべての API モデルに対する完全な型定義
  - 認証済み（`DefaultAPI`）とパブリック（`PublicAPI`）の両方のエンドポイント
  - 完全な async/await サポート
  - 詳細な API ドキュメントは [client/README.md](https://github.com/FastComments/fastcomments-swift/blob/main/client/README.md) を参照してください

- **SSO Module** - サーバーサイドのシングルサインオン用ユーティリティ
  - ユーザー認証のための安全なトークン生成
  - シンプルな SSO モードとセキュアな SSO モードの両方をサポート
  - CryptoKit を使用した HMAC-SHA256 ベースのトークン署名