### 401 Unauthorized エラー

認証済み API を使用しているときに 401 エラーが発生する場合:

1. **API キーを確認**: FastComments ダッシュボードから正しい API キーを使用していることを確認してください
2. **テナント ID を確認**: テナント ID がアカウントと一致していることを確認してください
3. **API キーの形式**: API キーは共有設定の `x-api-key` ヘッダーとして設定する必要があります:

```swift
FastCommentsSwiftAPIConfiguration.shared.customHeaders["x-api-key"] = "YOUR_API_KEY"
```

4. **間違った API の使用**: 認証済み呼び出しには `DefaultAPI`（`PublicAPI` ではなく）を使用していることを確認してください

### SSO トークンの問題

SSO トークンが機能しない場合:

1. **本番環境ではセキュアモードを使用**: 本番環境では常に API キーと共に `FastCommentsSSO.createSecure()` を使用してください
2. **サーバー側のみ**: セキュア SSO トークンはサーバー上で生成し、クライアントに API キーを公開しないでください
3. **ユーザーデータを確認**: 必要なフィールド（id、email、username）がすべて提供されていることを確認してください
4. **トークンの有効期限**: セキュア SSO トークンにはタイムスタンプが含まれ、期限切れになる可能性があります。必要に応じて新しいトークンを生成してください。

### SSL/TLS エラー

SSL/TLS エラーが発生した場合:

1. アプリの Info.plist が fastcomments.com への HTTPS 接続を許可していることを確認してください
2. 接続をブロックする可能性のある App Transport Security の例外を使用していないか確認してください