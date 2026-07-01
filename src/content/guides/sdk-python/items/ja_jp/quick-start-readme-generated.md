### 認証されたAPIの使用 (DefaultApi)

**重要:** 認証されたリクエストを行う前に、Configuration に API キーを設定する必要があります。設定しないと、リクエストは 401 エラーで失敗します。

```python
from client import ApiClient, Configuration, DefaultApi
from client.models import CreateAPISSOUserData

# APIクライアントを作成し、設定します
config = Configuration()
config.host = "https://fastcomments.com/api"

# 必須: API キーを設定します（FastComments ダッシュボードから取得してください）
config.api_key = {"ApiKeyAuth": "YOUR_API_KEY_HERE"}

# 設定したクライアントで API インスタンスを作成します
api_client = ApiClient(configuration=config)
api = DefaultApi(api_client)

# これで認証された API 呼び出しが可能です
try:
    # 例: SSO ユーザーを追加する
    user_data = CreateAPISSOUserData(
        id="user-123",
        email="user@example.com",
        display_name="John Doe"
    )

    response = api.add_sso_user("YOUR_TENANT_ID", user_data)
    print(f"User created: {response}")

except Exception as e:
    print(f"Error: {e}")
    # 一般的なエラー:
    # - 401: API キーが欠如または無効です
    # - 400: リクエストの検証に失敗しました
```

### パブリックAPIの使用 (PublicApi)

パブリックエンドポイントは認証を必要としません：

```python
from client import ApiClient, Configuration, PublicApi

config = Configuration()
config.host = "https://fastcomments.com/api"

api_client = ApiClient(configuration=config)
public_api = PublicApi(api_client)

try:
    response = public_api.get_comments_public("YOUR_TENANT_ID", "page-url-id")
    print(response)
except Exception as e:
    print(f"Error: {e}")
```

### モデレーションダッシュボードの使用 (ModerationApi)

`ModerationApi` はモデレーター用ダッシュボードを提供します。メソッドは `sso` トークンを渡すことでモデレーターとして呼び出されます：

```python
from client import ApiClient, Configuration, ModerationApi
from client.api.moderation_api import GetCountOptions

config = Configuration()
config.host = "https://fastcomments.com/api"

api_client = ApiClient(configuration=config)
moderation_api = ModerationApi(api_client)

try:
    # モデレーション待ちのコメント数を取得
    response = moderation_api.get_count(GetCountOptions(sso="SSO_TOKEN"))
    print(response)
except Exception as e:
    print(f"Error: {e}")
```

### SSO（シングルサインオン）の使用

SDK には安全な SSO トークンを生成するユーティリティが含まれています：

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

# このトークンをフロントエンドで使用するか、API 呼び出しに渡します
print(f"SSO Token: {sso_token}")
```

テスト用の簡易 SSO（セキュリティが低い）:

```python
from sso import FastCommentsSSO, SimpleSSOUserData

user_data = SimpleSSOUserData(
    user_id="user-123",
    email="user@example.com"
)

sso = FastCommentsSSO.new_simple(user_data)
sso_token = sso.create_token()
```

### 共通の問題

1. **401 "missing-api-key" エラー**: DefaultApi インスタンスを作成する前に、`config.api_key = {"ApiKeyAuth": "YOUR_KEY"}` を設定していることを確認してください。
2. **間違った API クラス**: サーバー側の認証リクエストには `DefaultApi`、クライアント側/パブリックリクエストには `PublicApi`、モデレーター用ダッシュボードのリクエストには `ModerationApi` を使用してください。
3. **インポートエラー**: 正しいモジュールからインポートしていることを確認してください:
   - API クライアント: `from client import ...`
   - SSO ユーティリティ: `from sso import ...`