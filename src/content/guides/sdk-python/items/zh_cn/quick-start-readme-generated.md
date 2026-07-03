### 使用已认证的 API（DefaultApi）

**重要提示：** 在进行已认证请求之前，必须在 Configuration 上设置您的 API 密钥。如果不设置，请求将会返回 401 错误。

```python
from client import ApiClient, Configuration, DefaultApi
from client.models import CreateAPISSOUserData

# 创建并配置 API 客户端
config = Configuration()
config.host = "https://fastcomments.com"

# 必填：设置您的 API 密钥（可在 FastComments 仪表板获取）
config.api_key = {"api_key": "YOUR_API_KEY_HERE"}

# 使用已配置的客户端创建 API 实例
api_client = ApiClient(configuration=config)
api = DefaultApi(api_client)

# 现在您可以进行已认证的 API 调用
try:
    # 示例：添加一个 SSO 用户
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
    # - 401: API 密钥缺失或无效
    # - 400: 请求验证失败
```

### 使用公共 API（PublicApi）

公共端点不需要身份验证：

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

`ModerationApi` 为审核员仪表板提供支持。通过传递 `sso` 令牌，以审核员身份调用方法：

```python
from client import ApiClient, Configuration, ModerationApi
from client.api.moderation_api import GetCountOptions

config = Configuration()
config.host = "https://fastcomments.com"

api_client = ApiClient(configuration=config)
moderation_api = ModerationApi(api_client)

try:
    # 统计等待审核的评论数量
    response = moderation_api.get_count(GetCountOptions(sso="SSO_TOKEN"))
    print(response)
except Exception as e:
    print(f"Error: {e}")
```

### 使用 SSO（单点登录）

SDK 包含生成安全 SSO 令牌的实用工具：

```python
from sso import FastCommentsSSO, SecureSSOUserData

# 创建用户数据（必须包含 id、email 和 username）
user_data = SecureSSOUserData(
    id="user-123",
    email="user@example.com",
    username="johndoe",
    avatar="https://example.com/avatar.jpg"
)

# 使用您的 API 密钥（HMAC-SHA256）对其进行签名
sso = FastCommentsSSO.new_secure("YOUR_API_SECRET", user_data)

# 生成可传递给小部件或 API 调用的 SSO 令牌
sso_token = sso.create_token()

# 在前端使用此令牌或传递给 API 调用
print(f"SSO Token: {sso_token}")
```

对于简单 SSO（安全性稍低，仅用于测试）：

```python
from sso import FastCommentsSSO, SimpleSSOUserData

user_data = SimpleSSOUserData(
    username="johndoe",
    email="user@example.com"
)

sso = FastCommentsSSO.new_simple(user_data)
sso_token = sso.create_token()
```

### 常见问题

1. **401 “missing-api-key” 错误**：确保在创建 `DefaultApi` 实例之前设置 `config.api_key = {"api_key": "YOUR_KEY"}`。
2. **错误的 API 类**：使用 `DefaultApi` 进行服务器端已认证请求，`PublicApi` 用于客户端/公共请求，`ModerationApi` 用于审核员仪表板请求。
3. **导入错误**：确保从正确的模块导入：
   - API 客户端：`from client import ...`
   - SSO 实用工具：`from sso import ...`