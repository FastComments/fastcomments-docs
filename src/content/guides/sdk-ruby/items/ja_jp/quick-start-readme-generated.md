### 認証済みAPIの使用 (DefaultApi)

**重要:** 認証付きリクエストを行う前に ApiClient に API キーを設定する必要があります。設定しないと、リクエストは 401 エラーで失敗します。

```ruby
require 'fastcomments'

# API クライアントの作成と設定
config = FastCommentsClient::Configuration.new
api_client = FastCommentsClient::ApiClient.new(config)

# 必須: API キーを設定してください (FastComments ダッシュボードから取得)
config.api_key['x-api-key'] = 'YOUR_API_KEY_HERE'

# 設定済みクライアントで API インスタンスを作成
api = FastCommentsClient::DefaultApi.new(api_client)

# これで認証付きの API 呼び出しが可能になります
begin
  # 例: SSO ユーザーを追加
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
  # - 401: API キーが存在しないか無効です
  # - 400: リクエストの検証に失敗しました
end
```

### パブリックAPIの使用 (PublicApi)

パブリックエンドポイントは認証を必要としません:

```ruby
require 'fastcomments'

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

### モデレーションAPIの使用 (ModerationApi)

モデレーション用のメソッドはモデレーターダッシュボードを動かします。リクエストを SSO 認証されたモデレータの代理として行うために、`sso` トークンを渡してください:

```ruby
require 'fastcomments'

moderation_api = FastCommentsClient::ModerationApi.new

begin
  # 例: モデレーションキューのコメントを一覧表示
  response = moderation_api.get_api_comments(
    sso: 'YOUR_MODERATOR_SSO_TOKEN'
  )
  puts response
rescue FastCommentsClient::ApiError => e
  puts e.message
end
```

### よくある問題

1. **401 "missing-api-key" エラー**: DefaultApi インスタンスを作成する前に `config.api_key['x-api-key'] = 'YOUR_KEY'` を設定していることを確認してください。
2. **Wrong API class**: サーバーサイドの認証付きリクエストには `DefaultApi`、クライアント側/パブリックリクエストには `PublicApi`、モデレーターダッシュボードのリクエストには `ModerationApi` を使用してください。
3. **Null API key**: API キーが null の場合、SDK は認証を黙ってスキップするため、401 エラーにつながります。