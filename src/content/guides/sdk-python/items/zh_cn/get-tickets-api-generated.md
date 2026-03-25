## 参数

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| userId | string | query | 否 |  |
| state | number | query | 否 |  |
| skip | number | query | 否 |  |
| limit | number | query | 否 |  |

## 响应

返回：[`GetTickets200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_tickets200_response.py)

## 示例

[inline-code-attrs-start title = 'get_tickets 示例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_tickets200_response import GetTickets200Response
from client.rest import ApiException
from pprint import pprint

# 定义主机是可选的，默认为 https://fastcomments.com
# 有关所有受支持的配置参数列表，请参阅 configuration.py。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# 客户端必须根据 API 服务器的安全策略配置身份验证和授权参数。
# 以下为每种身份验证方法提供示例，请使用符合您身份验证用例的示例。

# 配置 API 密钥授权：api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 如有需要，取消注释下面的行以为 API 密钥设置前缀（例如 Bearer）
# configuration.api_key_prefix['api_key'] = 'Bearer'

# 在带有 API 客户端实例的上下文中使用
with client.ApiClient(configuration) as api_client:
    # 创建 API 类的实例
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    user_id = 'user_id_example' # str |  (可选)
    state = 3.4 # float |  (可选)
    skip = 3.4 # float |  (可选)
    limit = 3.4 # float |  (可选)

    try:
        api_response = api_instance.get_tickets(tenant_id, user_id=user_id, state=state, skip=skip, limit=limit)
        print("The response of DefaultApi->get_tickets:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_tickets: %s\n" % e)
[inline-code-end]