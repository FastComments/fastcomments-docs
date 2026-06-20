The FastComments SDK は 3つのAPIクライアントを提供します:

### PublicAPI - Client-Safe Methods

The `PublicAPI` contains methods that are safe to call from client-side code (iOS/macOS apps). These methods:
- APIキーを必要としません
- 認証にSSOトークンを使用できます
- ユーザー/デバイスごとにレート制限されます
- エンドユーザー向けのアプリケーションに適しています

**Example use case**: iOSアプリでコメントを取得・作成する

### DefaultAPI - Server-Side Methods

The `DefaultAPI` contains authenticated methods that require an API key. These methods:
- FastCommentsのAPIキーが必要です
- サーバー側のコードからのみ呼び出す必要があります
- FastComments のデータへの完全なアクセスを提供します
- テナントごとにレート制限されます

**Example use case**: 管理操作、データの一括エクスポート、ユーザー管理

### ModerationAPI - Moderator Dashboard Methods

The `ModerationAPI` contains methods that power the moderator dashboard. These methods cover:
- **Comment moderation** - コメントの一覧、件数取得、検索、ログ取得、コメントのエクスポート
- **Moderation actions** - コメントの削除/復元、フラグ付け、レビュー/スパム/承認ステータスの設定、投票の管理、スレッドの再開/クローズ
- **Bans** - コメントからユーザーを禁止、禁止の取り消し、事前禁止サマリーの取得、禁止状況と設定の確認、禁止ユーザー数の読み取り
- **Badges & trust** - バッジの付与/削除、手動バッジの一覧、ユーザーの信頼度（trust factor）の取得/設定、ユーザーの内部プロフィールの読み取り

Every `ModerationAPI` method accepts an `sso` parameter so moderators can be authenticated via SSO.

**Example use case**: コミュニティのモデレーター向けにモデレーション機能を構築する場合

**IMPORTANT**: クライアント側のコードにAPIキーを絶対に露出しないでください。APIキーはサーバー側でのみ使用するべきです。