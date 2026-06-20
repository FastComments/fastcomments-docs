FastComments Rust SDK はいくつかのモジュールで構成されています：

- **Client Module** - FastComments REST API 用の API クライアント
  - すべてのAPIモデルの完全な型定義
  - FastComments のすべてのメソッドをカバーする3つのAPIクライアント：
    - `default_api` (**DefaultApi**) - サーバー側での使用向けの API キー認証されたメソッド
    - `public_api` (**PublicApi**) - パブリックで、API キー不要のメソッド。ブラウザやモバイルアプリから呼び出しても安全です
    - `moderation_api` (**ModerationApi**) - モデレータダッシュボードを支えるメソッド群（コメントのモデレーション（一覧、集計、検索、ログ、エクスポート）、モデレーションアクション（削除/復元、フラグ、レビュー/スパム/承認ステータスの設定、投票、スレッドの再開/クローズ）、バン（コメントからのバン、取り消し、事前バンの要約、バン状況/設定、バンされたユーザーの集計）、バッジと信頼（バッジの付与/削除、手動バッジ、信頼係数の取得/設定、ユーザー内部プロファイル））。すべての Moderation メソッドは SSO 認証されたモデレータを代理して呼び出せるように `sso` パラメータを受け取ります。
  - tokio を使った完全な async/await サポート
  - 詳細なAPIドキュメントは [client/README.md](https://github.com/FastComments/fastcomments-rust/blob/main/client/README.md) を参照してください

- **SSO Module** - サーバー側シングルサインオンユーティリティ
  - ユーザー認証のための安全なトークン生成
  - シンプルおよびセキュアの両方の SSO モードをサポート
  - HMAC-SHA256 によるトークン署名

- **Core Types** - 共有の型定義とユーティリティ
  - コメントモデルとメタデータ構造
  - ユーザーおよびテナントの設定
  - 一般的な操作のためのヘルパー関数