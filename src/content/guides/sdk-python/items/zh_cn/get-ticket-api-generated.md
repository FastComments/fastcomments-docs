## 参数

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| id | string | path | 是 |  |
| userId | string | query | 否 |  |

## 响应

返回: [`GetTicketResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_ticket_response.py)

## 示例

[inline-code-attrs-start title = 'get_ticket 示例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_ticket_response import GetTicketResponse
from client.rest import ApiException
from pprint import pprint

# 定义 host 是可选的，默认值为 https://fastcomments.com
# 有关所有支持的配置参数列表，请参阅 configuration.py。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# 客户端必须配置身份验证和授权参数
# 以遵守 API 服务器的安全策略。
# 为每种认证方法提供了示例，请使用
# 满足您认证用例的那个示例。

# 配置 API 密钥授权：api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 如有需要，取消注释下面以为 API 密钥设置前缀（例如 Bearer）
# configuration.api_key_prefix['api_key'] = 'Bearer'

# 使用 API 客户端实例进入上下文
with client.ApiClient(configuration) as api_client:
    # 创建 API 类的实例
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    user_id = 'user_id_example' # str |  (可选)

    try:
        api_response = api_instance.get_ticket(tenant_id, id, user_id=user_id)
        print("The response of DefaultApi->get_ticket:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_ticket: %s\n" % e)
[inline-code-end]