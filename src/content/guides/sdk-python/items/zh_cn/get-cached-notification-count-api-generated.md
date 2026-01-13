## 参数

| 名称 | 类型 | 位置 | 必需 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | 查询 | 是 |  |
| id | string | 路径 | 是 |  |

## 响应

返回: [`GetCachedNotificationCount200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_cached_notification_count200_response.py)

## 示例

[inline-code-attrs-start title = 'get_cached_notification_count 示例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_cached_notification_count200_response import GetCachedNotificationCount200Response
from client.rest import ApiException
from pprint import pprint

# 定义主机是可选的，默认值为 https://fastcomments.com
# 请参阅 configuration.py 以获取所有受支持的配置参数列表。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# 客户端必须配置身份验证和授权参数
# 以遵循 API 服务器的安全策略。
# 每种认证方法的示例在下面给出，请使用
# 满足您认证用例的示例。

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 如有需要，请取消注释下面内容以设置 API 密钥前缀（例如 Bearer）
# configuration.api_key_prefix['api_key'] = 'Bearer'

# 使用 API 客户端实例进入上下文
with client.ApiClient(configuration) as api_client:
    # 创建 API 类的一个实例
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 

    try:
        api_response = api_instance.get_cached_notification_count(tenant_id, id)
        print("The response of DefaultApi->get_cached_notification_count:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_cached_notification_count: %s\n" % e)
[inline-code-end]