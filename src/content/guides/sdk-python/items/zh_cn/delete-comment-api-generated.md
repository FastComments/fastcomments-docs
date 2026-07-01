## 参数

| 名称 | 类型 | 位置 | 必需 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |
| contextUserId | string | query | No |  |
| isLive | boolean | query | No |  |

## 响应

返回: [`DeleteCommentResult`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/delete_comment_result.py)

## 示例

[inline-code-attrs-start title = 'delete_comment 示例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.default_api import DeleteCommentOptions
from client.models.delete_comment_result import DeleteCommentResult
from client.rest import ApiException
from pprint import pprint

# 定义主机是可选的，默认值为 https://fastcomments.com
# 请参阅 configuration.py 获取所有支持的配置参数列表。
# 客户端必须配置身份验证和授权参数
# 以符合 API 服务器的安全策略。
# 为每种身份验证方法提供了下面的示例，使用
# 满足您身份验证使用情况的示例。

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 如有需要，取消注释以下代码以设置 API 密钥的前缀（例如 Bearer）
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Enter a context with an instance of the API client
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    context_user_id = 'context_user_id_example' # str |  (optional)
    is_live = True # bool |  (optional)

    try:
        api_response = api_instance.delete_comment(tenant_id, id, DeleteCommentOptions(context_user_id=context_user_id, is_live=is_live))
        print("The response of DefaultApi->delete_comment:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->delete_comment: %s\n" % e)
[inline-code-end]