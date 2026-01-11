### 401 Unauthorized エラー

認証された API を使用している際に 401 エラーが発生する場合:

1. **API key を確認**: FastComments ダッシュボードの正しい API key を使用していることを確認してください
2. **tenant ID を確認**: tenant ID がアカウントに一致していることを確認してください
3. **API key の設定方法**: API key は API クライアントに設定する必要があります:

```swift
let defaultApi = DefaultAPI()
defaultApi.apiKey = "YOUR_API_KEY"
```

4. **誤った API を使用していないか**: 認証が必要な呼び出しでは `DefaultAPI` （`PublicAPI` ではない）を使用していることを確認してください

### SSO トークンの問題

SSO トークンが機能しない場合:

1. **本番では secure mode を使用**: 本番環境では常に `FastCommentsSSO.createSecure()` を API key とともに使用してください
2. **サーバー側のみで生成**: セキュアな SSO トークンはサーバーで生成し、API key をクライアントに露出しないでください
3. **ユーザーデータを確認**: 必要なすべてのフィールド (id, email, username) が提供されていることを確認してください
4. **トークンの有効期限**: セキュアな SSO トークンにはタイムスタンプが含まれ、期限切れになる可能性があります。必要に応じて新しいトークンを生成してください。

### SSL/TLS エラー

SSL/TLS エラーが発生した場合:

1. アプリの Info.plist が fastcomments.com への HTTPS 接続を許可していることを確認してください
2. 接続をブロックする可能性のある App Transport Security の例外を使用していないか確認してください