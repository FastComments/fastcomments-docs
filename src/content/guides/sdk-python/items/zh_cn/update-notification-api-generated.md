## 参数

| 名称 | 类型 | 位置 | 必需 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| id | string | path | 是 |  |
| userId | string | query | 否 |  |

## 响应

返回：[`APIEmptyResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_empty_response.py)

## 示例

[inline-code-attrs-start title = 'update_notification 示例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.api_empty_response import APIEmptyResponse
from client.models.update_notification_body import UpdateNotificationBody
from client.rest import ApiException
from pprint import pprint

# 定义主机是可选的，默认值为 https://fastcomments.com
# 有关所有支持的配置参数的列表，请参见 configuration.py。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# 客户端必须根据 API 服务器的安全策略配置身份验证和授权参数。
# 以下为每种身份验证方法提供示例，使用满足您身份验证用例的示例。
# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 如有需要，取消注释下面的行以为 API 密钥设置前缀（例如 Bearer）。
# configuration.api_key_prefix['api_key'] = 'Bearer'

# 使用 API 客户端实例进入上下文
with client.ApiClient(configuration) as api_client:
    # 创建 API 类的一个实例
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    update_notification_body = client.UpdateNotificationBody() # UpdateNotificationBody | 
    user_id = 'user_id_example' # str |  (optional)

    try:
        api_response = api_instance.update_notification(tenant_id, id, update_notification_body, user_id=user_id)
        print("The response of DefaultApi->update_notification:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->update_notification: %s\n" % e)
[inline-code-end]