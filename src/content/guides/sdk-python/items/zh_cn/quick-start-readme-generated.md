### 使用已认证的 API (DefaultApi)

**重要：** 在进行已认证的请求之前，您必须在 Configuration 上设置您的 API 密钥。如果不设置，请求将以 401 错误失败。

```python
from client import ApiClient, Configuration, DefaultApi
from client.models import CreateAPISSOUserData

# 创建并配置 API 客户端
config = Configuration()
config.host = "https://fastcomments.com/api"

# 必填：设置你的 API 密钥（从 FastComments 仪表板获取）
config.api_key = {"ApiKeyAuth": "YOUR_API_KEY_HERE"}

# 使用配置好的客户端创建 API 实例
api_client = ApiClient(configuration=config)
api = DefaultApi(api_client)

# 现在你可以进行已认证的 API 调用
try:
    # 示例：添加 SSO 用户
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
    # 常见错误：
    # - 401：API 密钥丢失或无效
    # - 400：请求验证失败
```

### 使用公共 API (PublicApi)

公共端点不需要认证：

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

### 使用 SSO（单点登录）

该 SDK 包含用于生成安全 SSO 令牌的实用工具：

```python
from sso import FastCommentsSSO, SecureSSOUserData

# 创建用户数据
user_data = SecureSSOUserData(
    user_id="user-123",
    email="user@example.com",
    username="johndoe",
    avatar="https://example.com/avatar.jpg"
)

# 使用你的 API 密钥（secret）创建 SSO 实例
sso = FastCommentsSSO.new_secure(
    api_secret="YOUR_API_SECRET",
    user_data=user_data
)

# 生成 SSO 令牌
sso_token = sso.create_token()

# 在前端使用此令牌或传递给 API 调用
print(f"SSO Token: {sso_token}")
```

对于简单 SSO（不太安全，仅用于测试）：

```python
from sso import FastCommentsSSO, SimpleSSOUserData

user_data = SimpleSSOUserData(
    user_id="user-123",
    email="user@example.com"
)

sso = FastCommentsSSO.new_simple(user_data)
sso_token = sso.create_token()
```

### 常见问题

1. **401 "missing-api-key" 错误**：确保在创建 DefaultApi 实例之前设置 `config.api_key = {"ApiKeyAuth": "YOUR_KEY"}`。
2. **错误的 API 类**：对服务器端的已认证请求使用 `DefaultApi`，对客户端/公共请求使用 `PublicApi`。
3. **导入错误**：确保你从正确的模块导入：
   - API 客户端：`from client import ...`
   - SSO 实用工具：`from sso import ...`