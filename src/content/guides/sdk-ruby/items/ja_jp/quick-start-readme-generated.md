### 認証された API の使用 (DefaultApi)

**重要:** 認証されたリクエストを行う前に、ApiClient に API キーを設定する必要があります。設定しない場合、リクエストは 401 エラーで失敗します。

```ruby
require 'fastcomments'

# APIクライアントを作成し、設定する
config = FastCommentsClient::Configuration.new
api_client = FastCommentsClient::ApiClient.new(config)

# 必須: APIキーを設定します（FastComments ダッシュボードから取得）
config.api_key['x-api-key'] = 'YOUR_API_KEY_HERE'

# 設定されたクライアントで API インスタンスを作成する
api = FastCommentsClient::DefaultApi.new(api_client)

# これで認証された API 呼び出しができます
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
  # 一般的なエラー:
  # - 401: API キーが欠落しているか無効です
  # - 400: リクエストの検証に失敗しました
end
```

### パブリック API の使用 (PublicApi)

パブリックエンドポイントは認証を必要としません：

```ruby
require 'fastcomments'

public_api = FastCommentsClient::PublicApi.new

begin
  response = public_api.get_comments_public(
    'YOUR_TENANT_ID',
    'page-url-id'
  )
  puts response
rescue FastCommentsClient::ApiError => e
  puts e.message
end
```

### モデレーション API の使用 (ModerationApi)

モデレーションメソッドはモデレーター ダッシュボードを動かします。`sso` トークンを渡すことで、SSO 認証されたモデレーターの代理でリクエストが行われます：

```ruby
require 'fastcomments'

moderation_api = FastCommentsClient::ModerationApi.new

begin
  # 例: モデレーションキュー内のコメントを一覧取得
  response = moderation_api.get_api_comments(
    sso: 'YOUR_MODERATOR_SSO_TOKEN'
  )
  puts response
rescue FastCommentsClient::ApiError => e
  puts e.message
end
```

### 共通の問題

1. **401 "missing-api-key" エラー**: `DefaultApi` インスタンスを作成する前に `config.api_key['x-api-key'] = 'YOUR_KEY'` を設定していることを確認してください。  
2. **誤った API クラス**: サーバー側の認証リクエストには `DefaultApi`、クライアント側/パブリックリクエストには `PublicApi`、モデレーター ダッシュボードのリクエストには `ModerationApi` を使用してください。  
3. **NULL の API キー**: SDK は API キーが null の場合、認証を黙ってスキップし、結果として 401 エラーが発生します。