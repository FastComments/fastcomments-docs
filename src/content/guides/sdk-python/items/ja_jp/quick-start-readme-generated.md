### 認証済み API の使用 (DefaultApi)

**重要:** 認証されたリクエストを行う前に、Configuration に API キーを設定する必要があります。設定していない場合、リクエストは 401 エラーで失敗します。

```python
from client import ApiClient, Configuration, DefaultApi
from client.models import CreateAPISSOUserData

# API クライアントを作成して設定
config = Configuration()
config.host = "https://fastcomments.com/api"

# 必須: API キーを設定してください（FastComments のダッシュボードから取得）
config.api_key = {"ApiKeyAuth": "YOUR_API_KEY_HERE"}

# 設定済みのクライアントで API インスタンスを作成
api_client = ApiClient(configuration=config)
api = DefaultApi(api_client)

# これで認証済みの API 呼び出しが可能です
try:
    # 例: SSO ユーザーを追加
    user_data = CreateAPISSOUserData(
        id="user-123",
        email="user@example.com",
        display_name="John Doe"
    )

    response = api.add_sso_user(
        tenant_id="YOUR_TENANT_ID",
        create_apisso_user_data=user_data
    )
    print(f"User created: {response}")

except Exception as e:
    print(f"Error: {e}")
    # よくあるエラー:
    # - 401: API キーがないか無効です
    # - 400: リクエストの検証に失敗しました
```

### パブリック API の使用 (PublicApi)

パブリックエンドポイントは認証不要です:

```python
from client import ApiClient, Configuration, PublicApi

config = Configuration()
config.host = "https://fastcomments.com/api"

api_client = ApiClient(configuration=config)
public_api = PublicApi(api_client)

try:
    response = public_api.get_comments_public(
        tenant_id="YOUR_TENANT_ID",
        url_id="page-url-id"
    )
    print(response)
except Exception as e:
    print(f"Error: {e}")
```

### モデレーションダッシュボードの使用 (ModerationApi)

The `ModerationApi` powers the moderator dashboard. Methods are called on behalf of a moderator by passing an `sso` token:

```python
from client import ApiClient, Configuration, ModerationApi

config = Configuration()
config.host = "https://fastcomments.com/api"

api_client = ApiClient(configuration=config)
moderation_api = ModerationApi(api_client)

try:
    # モデレーション待ちのコメント数をカウントする
    response = moderation_api.get_count(sso="SSO_TOKEN")
    print(response)
except Exception as e:
    print(f"Error: {e}")
```

### SSO (Single Sign-On) の使用

SDK には安全な SSO トークンを生成するユーティリティが含まれています:

```python
from sso import FastCommentsSSO, SecureSSOUserData

# ユーザーデータを作成
user_data = SecureSSOUserData(
    user_id="user-123",
    email="user@example.com",
    username="johndoe",
    avatar="https://example.com/avatar.jpg"
)

# API シークレットで SSO インスタンスを作成
sso = FastCommentsSSO.new_secure(
    api_secret="YOUR_API_SECRET",
    user_data=user_data
)

# SSO トークンを生成
sso_token = sso.create_token()

# このトークンをフロントエンドで使用するか API 呼び出しに渡す
print(f"SSO Token: {sso_token}")
```

簡易 SSO（安全性は低く、テスト用）:

```python
from sso import FastCommentsSSO, SimpleSSOUserData

user_data = SimpleSSOUserData(
    user_id="user-123",
    email="user@example.com"
)

sso = FastCommentsSSO.new_simple(user_data)
sso_token = sso.create_token()
```

### よくある問題

1. **401 "missing-api-key" error**: DefaultApi インスタンスを作成する前に `config.api_key = {"ApiKeyAuth": "YOUR_KEY"}` を設定していることを確認してください。
2. **API クラスの間違い**: サーバー側の認証済みリクエストには `DefaultApi` を、クライアント側/パブリックなリクエストには `PublicApi` を、モデレーターダッシュボードのリクエストには `ModerationApi` を使用してください。
3. **インポートエラー**: 正しいモジュールからインポートしていることを確認してください:
   - API クライアント: `from client import ...`
   - SSO ユーティリティ: `from sso import ...`