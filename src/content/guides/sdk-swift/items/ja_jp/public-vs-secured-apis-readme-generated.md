The FastComments SDK provides two types of API endpoints:

### PublicAPI - クライアント向け安全エンドポイント

The `PublicAPI` contains endpoints that are safe to call from client-side code (iOS/macOS apps). These endpoints:
- API key を必要としません
- 認証に SSO tokens を使用できます
- ユーザー/デバイスごとにレート制限されます
- エンドユーザー向けアプリケーションに適しています

**Example use case**: iOSアプリでのコメントの取得と作成

### DefaultAPI - サーバーサイドエンドポイント

The `DefaultAPI` contains authenticated endpoints that require an API key. These endpoints:
- あなたの FastComments API key が必要です
- サーバー側のコードからのみ呼び出すべきです
- あなたの FastComments データへのフルアクセスを提供します
- テナントごとにレート制限されます

**Example use case**: 管理操作、大量データのエクスポート、モデレーションツール

**IMPORTANT**: クライアント側のコードに API key を決して公開しないでください。API keys はサーバー側でのみ使用してください。