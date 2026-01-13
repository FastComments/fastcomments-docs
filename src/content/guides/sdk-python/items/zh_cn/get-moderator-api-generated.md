## 参数

| 名称 | 类型 | 位置 | 是否必需 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| id | string | path | 是 |  |

## 响应

返回: [`GetModerator200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_moderator200_response.py)

## 示例

[inline-code-attrs-start title = 'get_moderator 示例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_moderator200_response import GetModerator200Response
from client.rest import ApiException
from pprint import pprint

# 定义主机是可选的，默认为 https://fastcomments.com
# 有关所有支持的配置参数列表，请参阅 configuration.py。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# 客户端必须配置认证和授权参数
# 以符合 API 服务器的安全策略。
# 为每种认证方法提供了示例，请选择满足您认证用例的示例。

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
# configuration.api_key_prefix['api_key'] = 'Bearer'

# 使用 API 客户端实例进入上下文
with client.ApiClient(configuration) as api_client:
    # 创建 API 类的一个实例
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 

    try:
        api_response = api_instance.get_moderator(tenant_id, id)
        print("The response of DefaultApi->get_moderator:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_moderator: %s\n" % e)
[inline-code-end]