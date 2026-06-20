## 参数

| 名称 | 类型 | 位置 | 必需 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| urlId | string | query | 是 |  |
| userId | string | query | 否 |  |
| anonUserId | string | query | 否 |  |

## 响应

返回: [`GetVotesForUserResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_votes_for_user_response.py)

## 示例

[inline-code-attrs-start title = 'get_votes_for_user 示例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_votes_for_user_response import GetVotesForUserResponse
from client.rest import ApiException
from pprint import pprint

# 定义主机是可选的，默认值为 https://fastcomments.com
# 有关所有受支持的配置参数列表，请参阅 configuration.py。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# 客户端必须配置身份验证和授权参数
# 以符合 API 服务器的安全策略。
# 以下提供了每种身份验证方法的示例，使用满足您身份验证用例的示例。
#
# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 如有需要，取消注释下面以设置 API 密钥前缀（例如 Bearer）
# configuration.api_key_prefix['api_key'] = 'Bearer'

# 进入包含 API 客户端实例的上下文
with client.ApiClient(configuration) as api_client:
    # 创建 API 类的实例
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | 
    user_id = 'user_id_example' # str |  (可选)
    anon_user_id = 'anon_user_id_example' # str |  (可选)

    try:
        api_response = api_instance.get_votes_for_user(tenant_id, url_id, user_id=user_id, anon_user_id=anon_user_id)
        print("The response of DefaultApi->get_votes_for_user:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_votes_for_user: %s\n" % e)
[inline-code-end]