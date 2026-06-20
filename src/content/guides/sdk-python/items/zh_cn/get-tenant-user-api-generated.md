## 参数

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| id | string | path | 是 |  |

## Response

返回：[`GetTenantUserResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_tenant_user_response.py)

## 示例

[inline-code-attrs-start title = 'get_tenant_user 示例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_tenant_user_response import GetTenantUserResponse
from client.rest import ApiException
from pprint import pprint

# 定义主机是可选的，默认值为 https://fastcomments.com
# 有关所有支持的配置参数列表，请参见 configuration.py。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# 客户端必须配置身份验证和授权参数
# 以符合 API 服务器的安全策略。
# 下方提供了每种身份验证方法的示例，
# 使用满足你身份验证用例的示例。

# 配置 API 密钥授权：api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 如有需要，取消注释下面的代码以为 API 密钥设置前缀（例如 Bearer）
# configuration.api_key_prefix['api_key'] = 'Bearer'

# 使用 API 客户端实例进入上下文
with client.ApiClient(configuration) as api_client:
    # 创建 API 类的一个实例
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 

    try:
        api_response = api_instance.get_tenant_user(tenant_id, id)
        print("The response of DefaultApi->get_tenant_user:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_tenant_user: %s\n" % e)
[inline-code-end]