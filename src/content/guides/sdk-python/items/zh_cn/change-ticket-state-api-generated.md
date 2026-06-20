## 参数

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| userId | string | query | 是 |  |
| id | string | path | 是 |  |

## 响应

返回: [`ChangeTicketStateResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/change_ticket_state_response.py)

## 示例

[inline-code-attrs-start title = 'change_ticket_state 示例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.change_ticket_state_body import ChangeTicketStateBody
from client.models.change_ticket_state_response import ChangeTicketStateResponse
from client.rest import ApiException
from pprint import pprint

# 定义 host 是可选的，默认值为 https://fastcomments.com
# 有关所有支持的配置参数列表，请参见 configuration.py。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# 客户端必须配置身份验证和授权参数
# 以符合 API 服务器的安全策略。
# 下面为每种认证方法提供了示例，请使用满足您认证用例的示例。

# 配置 API 密钥授权：api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 如果需要，请取消注释下面以设置前缀（例如 Bearer）用于 API 密钥
# configuration.api_key_prefix['api_key'] = 'Bearer'

# 在 API 客户端实例的上下文中
with client.ApiClient(configuration) as api_client:
    # 创建 API 类的一个实例
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    user_id = 'user_id_example' # str | 
    id = 'id_example' # str | 
    change_ticket_state_body = client.ChangeTicketStateBody() # ChangeTicketStateBody | 

    try:
        api_response = api_instance.change_ticket_state(tenant_id, user_id, id, change_ticket_state_body)
        print("The response of DefaultApi->change_ticket_state:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->change_ticket_state: %s\n" % e)
[inline-code-end]