### 使用已认证的 API（DefaultApi）

**重要提示：** 在进行已认证请求之前，您必须在 Configuration 上设置您的 API 密钥。如果不设置，请求将会返回 401 错误。

```python
from client import ApiClient, Configuration, DefaultApi
from client.models import CreateAPISSOUserData

# 创建并配置 API 客户端
config = Configuration()
config.host = "https://fastcomments.com"

# 必填：设置您的 API 密钥（从 FastComments 仪表板获取）
config.api_key = {"api_key": "YOUR_API_KEY_HERE"}

# 使用已配置的客户端创建 API 实例
api_client = ApiClient(configuration=config)
api = DefaultApi(api_client)

# 现在您可以进行已认证的 API 调用
try:
    # 示例：添加 SSO 用户
    user_data = CreateAPISSOUserData(
        id="user-123",
        email="user@example.com",
        display_name="John Doe"
    )

    response = api.add_sso_user("YOUR_TENANT_ID", user_data)
    print(f"User created: {response}")

except Exception as e:
    print(f"Error: {e}")
    # 常见错误：
    # - 401：缺少或无效的 API 密钥
    # - 400：请求验证失败
```

### 使用公共 API（PublicApi）

公共端点不需要认证：

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

### 使用审核仪表板（ModerationApi）

`ModerationApi` 为审核员仪表板提供功能。通过传递 `sso` 令牌，以审核员身份调用方法：

```python
from client import ApiClient, Configuration, ModerationApi
from client.api.moderation_api import GetCountOptions

config = Configuration()
config.host = "https://fastcomments.com"

api_client = ApiClient(configuration=config)
moderation_api = ModerationApi(api_client)

try:
    # 统计等待审核的评论
    response = moderation_api.get_count(GetCountOptions(sso="SSO_TOKEN"))
    print(response)
except Exception as e:
    print(f"Error: {e}")
```

### 使用 SSO（单点登录）

SDK 包含用于生成安全 SSO 令牌的工具：

```python
from sso import FastCommentsSSO, SecureSSOUserData

# 创建用户数据（id、email 和 username 为必填）
user_data = SecureSSOUserData(
    id="user-123",
    email="user@example.com",
    username="johndoe",
    avatar="https://example.com/avatar.jpg"
)

# 使用您的 API 密钥进行签名（HMAC-SHA256）
sso = FastCommentsSSO.new_secure("YOUR_API_SECRET", user_data)

# 生成 SSO 令牌以传递给小部件或 API 调用
sso_token = sso.create_token()

# 在前端使用此令牌或传递给 API 调用
print(f"SSO Token: {sso_token}")
```

对于简单 SSO（安全性较低，仅用于测试）：

```python
from sso import FastCommentsSSO, SimpleSSOUserData

user_data = SimpleSSOUserData(
    username="johndoe",
    email="user@example.com"
)

sso = FastCommentsSSO.new_simple(user_data)
sso_token = sso.create_token()
```

### 实时订阅（PubSub）

`pubsub` 模块允许您通过 WebSocket 订阅实时评论事件（新评论、投票、编辑、通知等），与 FastComments Java SDK 的 `LiveEventSubscriber` 相对应。它需要 `pubsub` 额外依赖，在生成的 API 客户端之上添加 WebSocket 客户端：

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
    user_id_ws="a-unique-presence-id",  # 例如，此会话的 UUID
    handle_live_event=handle_live_event,
    on_connection_status_change=lambda connected, last_event_time: print(
        f"connected={connected}"
    ),
    region=None,  # 设置为 "eu" 以使用欧盟地区
)

# ...稍后，当您不再需要更新时：
result.close()
```

订阅者在后台守护线程中运行连接，透明地进行抖动重连，并在重新连接时从 event‑log 端点获取在断开期间错过的任何事件。传入可选的 `can_see_comments` 回调（`List[str] -> Dict[str, str]`，返回用户不可见的 ID）以过滤用户无权查看的评论事件。将 `disable_live_commenting=True` 设置为使 `subscribe_to_changes` 成为返回 `None` 的空操作。

### 常见问题

1. **401 “missing-api-key” 错误**：确保在创建 DefaultApi 实例之前设置 `config.api_key = {"api_key": "YOUR_KEY"}`。
2. **错误的 API 类**：对服务器端已认证请求使用 `DefaultApi`，对客户端/公共请求使用 `PublicApi`，对审核员仪表板请求使用 `ModerationApi`。
3. **导入错误**：确保从正确的模块导入：
   - API 客户端：`from client import ...`
   - SSO 工具：`from sso import ...`
   - 实时订阅：`from pubsub import ...`（需要 `pubsub` 额外依赖）