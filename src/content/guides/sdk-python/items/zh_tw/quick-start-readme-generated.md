### 使用已驗證的 API（DefaultApi）

**重要：** 在發送已驗證的請求之前，您必須在 Configuration 上設定您的 API 金鑰。如果未設定，請求將會以 401 錯誤失敗。

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

### 使用公共 API（PublicApi）

公共端點不需要驗證：

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

### 使用審核儀表板（ModerationApi）

`ModerationApi` 為審核員儀表板提供功能。方法透過傳遞 `sso` 令牌，以審核員的身份呼叫：

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

### 使用 SSO（單一登入）

SDK 包含用於產生安全 SSO 令牌的工具：

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

簡易 SSO（較不安全，僅供測試）：

```python
from sso import FastCommentsSSO, SimpleSSOUserData

user_data = SimpleSSOUserData(
    username="johndoe",
    email="user@example.com"
)

sso = FastCommentsSSO.new_simple(user_data)
sso_token = sso.create_token()
```

### 即時訂閱（PubSub）

`pubsub` 模組讓您透過 WebSocket 訂閱即時評論事件（新評論、投票、編輯、通知等），類似 FastComments Java SDK 的 `LiveEventSubscriber`。它需要 `pubsub` 額外套件，會在產生的 API 客戶端上加入 WebSocket 客戶端：

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

訂閱者在背景守護執行緒上執行連線，會透明地以抖動方式重新連線，並在重新連線時從事件日誌端點抓取斷線期間遺失的任何事件。傳入可選的 `can_see_comments` 回呼函式（`List[str] -> Dict[str, str]`，回傳使用者不可見的 ID）以過濾使用者無權查看的評論事件。將 `disable_live_commenting=True` 設為 true，可使 `subscribe_to_changes` 成為不執行任何操作且回傳 `None` 的函式。

### 常見問題

1. **401 「missing-api-key」錯誤**：確保在建立 DefaultApi 實例之前，已設定 `config.api_key = {"api_key": "YOUR_KEY"}`。
2. **錯誤的 API 類別**：對於伺服器端已驗證的請求使用 `DefaultApi`，對於客戶端/公共請求使用 `PublicApi`，而審核員儀表板請求則使用 `ModerationApi`。
3. **匯入錯誤**：確保從正確的模組匯入：
   - API 客戶端：`from client import ...`
   - SSO 工具：`from sso import ...`
   - 即時訂閱：`from pubsub import ...`（需要 `pubsub` 額外套件）