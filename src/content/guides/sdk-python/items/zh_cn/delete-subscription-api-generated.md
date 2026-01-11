## 参数

| 名称 | 类型 | 位置 | 必需 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | 查询 | 是 |  |
| id | string | 路径 | 是 |  |
| userId | string | 查询 | 否 |  |

## 响应

返回: [`DeleteSubscriptionAPIResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/delete_subscription_api_response.py)

## 示例

[inline-code-attrs-start title = 'delete_subscription 示例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.delete_subscription_api_response import DeleteSubscriptionAPIResponse
from client.rest import ApiException
from pprint import pprint

# 定义主机是可选的，默认为 https://fastcomments.com
# 有关所有支持的配置参数，请参见 configuration.py。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# 客户端必须根据 API 服务器的安全策略配置身份验证和授权参数。
# 下面为每种身份验证方法提供了示例，请使用满足您身份验证用例的示例。

# 配置 API 密钥授权：api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 如果需要，请取消下面的注释以为 API 密钥设置前缀（例如 Bearer）
# configuration.api_key_prefix['api_key'] = 'Bearer'

# 使用 API 客户端实例进入上下文
with client.ApiClient(configuration) as api_client:
    # 创建 API 类的一个实例
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    user_id = 'user_id_example' # str |  (可选)

    try:
        api_response = api_instance.delete_subscription(tenant_id, id, user_id=user_id)
        print("The response of DefaultApi->delete_subscription:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->delete_subscription: %s\n" % e)
[inline-code-end]

---