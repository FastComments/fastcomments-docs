### 認証済み API の使用 (DefaultApi)

**重要:** 認証済みリクエストを行う前に、Configuration に API キーを設定する必要があります。設定しない場合、リクエストは 401 エラーで失敗します。

```python
from client import ApiClient, Configuration, DefaultApi
from client.models import CreateAPISSOUserData

# Create and configure the API client
config = Configuration()
config.host = "https://fastcomments.com"

# REQUIRED: Set your API key (get this from your FastComments dashboard)
config.api_key = {"api_key": "YOUR_API_KEY_HERE"}

# Create the API instance with the configured client
api_client = ApiClient(configuration=config)
api = DefaultApi(api_client)

# Now you can make authenticated API calls
try:
    # Example: Add an SSO user
    user_data = CreateAPISSOUserData(
        id="user-123",
        email="user@example.com",
        display_name="John Doe"
    )

    response = api.add_sso_user("YOUR_TENANT_ID", user_data)
    print(f"User created: {response}")

except Exception as e:
    print(f"Error: {e}")
    # Common errors:
    # - 401: API key is missing or invalid
    # - 400: Request validation failed
```

### パブリック API の使用 (PublicApi)

パブリックエンドポイントは認証を必要としません:

```python
from client import ApiClient, Configuration, PublicApi

config = Configuration()
config.host = "https://fastcomments.com"

api_client = ApiClient(configuration=config)
public_api = PublicApi(api_client)

try:
    response = public_api.get_comments_public("YOUR_TENANT_ID", "page-url-id")
    print(response)
except Exception as e:
    print(f"Error: {e}")
```

### モデレーターダッシュボードの使用 (ModerationApi)

`ModerationApi` はモデレーターダッシュボードを提供します。メソッドは `sso` トークンを渡すことでモデレーターの代理として呼び出されます:

```python
from client import ApiClient, Configuration, ModerationApi
from client.api.moderation_api import GetCountOptions

config = Configuration()
config.host = "https://fastcomments.com"

api_client = ApiClient(configuration=config)
moderation_api = ModerationApi(api_client)

try:
    # Count the comments awaiting moderation
    response = moderation_api.get_count(GetCountOptions(sso="SSO_TOKEN"))
    print(response)
except Exception as e:
    print(f"Error: {e}")
```

### SSO（シングルサインオン）の使用

SDK には安全な SSO トークンを生成するユーティリティが含まれています:

```python
from sso import FastCommentsSSO, SecureSSOUserData

# Create user data (id, email, and username are required)
user_data = SecureSSOUserData(
    id="user-123",
    email="user@example.com",
    username="johndoe",
    avatar="https://example.com/avatar.jpg"
)

# Sign it with your API secret (HMAC-SHA256)
sso = FastCommentsSSO.new_secure("YOUR_API_SECRET", user_data)

# Generate the SSO token to pass to the widget or an API call
sso_token = sso.create_token()

# Use this token in your frontend or pass to API calls
print(f"SSO Token: {sso_token}")
```

シンプル SSO（セキュリティが低く、テスト用）:

```python
from sso import FastCommentsSSO, SimpleSSOUserData

user_data = SimpleSSOUserData(
    username="johndoe",
    email="user@example.com"
)

sso = FastCommentsSSO.new_simple(user_data)
sso_token = sso.create_token()
```

### ライブサブスクリプション (PubSub)

`pubsub` モジュールを使用すると、WebSocket を介してリアルタイムのコメントイベント（新規コメント、投票、編集、通知など）を購読できます。これは FastComments の Java SDK の `LiveEventSubscriber` を鏡像しています。`pubsub` エクストラが必要で、生成された API クライアントに WebSocket クライアントを追加します:

```bash
pip install "fastcomments[pubsub] @ git+https://github.com/fastcomments/fastcomments-python.git@v3.1.0"
```

```python
from pubsub import LiveEventSubscriber

subscriber = LiveEventSubscriber()

def handle_live_event(event):
    print(f"Live event: {event.type}")
    if event.comment:
        print(f"  comment: {event.comment.comment}")

result = subscriber.subscribe_to_changes(
    tenant_id_ws="YOUR_TENANT_ID",
    url_id="page-url-id",
    url_id_ws="page-url-id",
    user_id_ws="a-unique-presence-id",  # e.g. a UUID for this session
    handle_live_event=handle_live_event,
    on_connection_status_change=lambda connected, last_event_time: print(
        f"connected={connected}"
    ),
    region=None,  # set to "eu" for the EU region
)

# ...later, when you no longer want updates:
result.close()
```

サブスクライバーはバックグラウンドのデーモンスレッドで接続を実行し、ジッター付きで透過的に再接続し、再接続時にイベントログエンドポイントから切断中に失われたイベントを取得します。オプションで `can_see_comments` コールバック（`List[str] -> Dict[str, str]`、ユーザーが閲覧できない ID を返す）を渡すことで、ユーザーが閲覧できないコメントのイベントをフィルタリングできます。`disable_live_commenting=True` を設定すると、`subscribe_to_changes` は何も行わず `None` を返すようになります。

### 共通の問題

1. **401 "missing-api-key" エラー**: DefaultApi インスタンスを作成する前に `config.api_key = {"api_key": "YOUR_KEY"}` を設定してください。
2. **API クラスの誤り**: サーバー側の認証リクエストには `DefaultApi`、クライアント側/パブリックリクエストには `PublicApi`、モデレーターダッシュボードのリクエストには `ModerationApi` を使用してください。
3. **インポートエラー**: 正しいモジュールからインポートしていることを確認してください:
   - API クライアント: `from client import ...`
   - SSO ユーティリティ: `from sso import ...`
   - ライブサブスクリプション: `from pubsub import ...`（`pubsub` エクストラが必要）