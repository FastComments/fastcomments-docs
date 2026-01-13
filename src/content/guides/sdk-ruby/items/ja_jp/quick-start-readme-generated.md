### 認証済みAPIの使用 (DefaultApi)

**重要:** 認証が必要なリクエストを行う前に、ApiClientにAPIキーを設定する必要があります。設定しないと、リクエストは401エラーで失敗します。

```ruby
require 'fastcomments-client'

# APIクライアントを作成して設定する
config = FastCommentsClient::Configuration.new
api_client = FastCommentsClient::ApiClient.new(config)

# 必須: APIキーを設定してください（FastCommentsのダッシュボードで取得）
config.api_key['x-api-key'] = 'YOUR_API_KEY_HERE'

# 設定済みのクライアントでAPIインスタンスを作成する
api = FastCommentsClient::DefaultApi.new(api_client)

# これで認証付きのAPI呼び出しが可能です
begin
  # 例: SSOユーザーを追加する
  user_data = {
    id: 'user-123',
    email: 'user@example.com',
    displayName: 'John Doe'
  }

  response = api.add_sso_user('YOUR_TENANT_ID', user_data)
  puts "User created: #{response}"

rescue FastCommentsClient::ApiError => e
  puts "Error: #{e.response_body}"
  # よくあるエラー:
  # - 401: APIキーが欠如しているか無効です
  # - 400: リクエストの検証に失敗しました
end
```

### 公開APIの使用 (PublicApi)

公開エンドポイントは認証を必要としません:

```ruby
require 'fastcomments-client'

public_api = FastCommentsClient::PublicApi.new

begin
  response = public_api.get_comments_public(
    tenant_id: 'YOUR_TENANT_ID',
    url_id: 'page-url-id'
  )
  puts response
rescue FastCommentsClient::ApiError => e
  puts e.message
end
```

### よくある問題

1. **401 "missing-api-key" error**: DefaultApiインスタンスを作成する前に `config.api_key['x-api-key'] = 'YOUR_KEY'` を設定していることを確認してください。
2. **Wrong API class**: サーバー側の認証済みリクエストには `DefaultApi` を使用し、クライアント側/公開リクエストには `PublicApi` を使用してください。
3. **Null API key**: APIキーがnullの場合、SDKは認証を静かにスキップし、401エラーが発生します。