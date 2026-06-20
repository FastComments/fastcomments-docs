## 参数

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| skip | number | query | 否 |  |

## 响应

返回: [`GetTenantUsersResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_tenant_users_response.py)

## 示例

[inline-code-attrs-start title = 'get_tenant_users 示例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_tenant_users_response import GetTenantUsersResponse
from client.rest import ApiException
from pprint import pprint

# 定义主机是可选的，默认值为 https://fastcomments.com
# 请查看 configuration.py 以获取所有支持的配置参数的列表。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# 客户端必须配置认证和授权参数
# 以遵守 API 服务器的安全策略。
# 下方为每种认证方法提供了示例，请选择
# 适合您认证用例的示例。
# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 如有需要，请取消注释下面以为 API 密钥设置前缀（例如 Bearer）
# configuration.api_key_prefix['api_key'] = 'Bearer'

# 使用 API 客户端实例进入上下文
with client.ApiClient(configuration) as api_client:
    # 创建 API 类的实例
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    skip = 3.4 # float |  (optional)

    try:
        api_response = api_instance.get_tenant_users(tenant_id, skip=skip)
        print("The response of DefaultApi->get_tenant_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_tenant_users: %s\n" % e)
[inline-code-end]