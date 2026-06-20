## 参数

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |
| userId | string | query | No |  |
| anonUserId | string | query | No |  |

## 响应

返回: [`BlockSuccess`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/block_success.py)

## 示例

[inline-code-attrs-start title = 'block_user_from_comment 示例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.block_from_comment_params import BlockFromCommentParams
from client.models.block_success import BlockSuccess
from client.rest import ApiException
from pprint import pprint

# 定义主机是可选的，默认为 https://fastcomments.com
# 请参阅 configuration.py 获取支持的所有配置参数的列表。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# 客户端必须配置认证和授权参数
# 以符合 API 服务器的安全策略。
# 下方为每种认证方法提供示例，
# 请使用满足您认证用例的示例。

# 配置 API 密钥认证：api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 如有需要，取消注释下面内容以为 API 密钥设置前缀（例如 Bearer）
# configuration.api_key_prefix['api_key'] = 'Bearer'

# 使用 API 客户端实例进入上下文
with client.ApiClient(configuration) as api_client:
    # 创建 API 类的一个实例
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    block_from_comment_params = client.BlockFromCommentParams() # BlockFromCommentParams | 
    user_id = 'user_id_example' # str |  (可选)
    anon_user_id = 'anon_user_id_example' # str |  (可选)

    try:
        api_response = api_instance.block_user_from_comment(tenant_id, id, block_from_comment_params, user_id=user_id, anon_user_id=anon_user_id)
        print("The response of DefaultApi->block_user_from_comment:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->block_user_from_comment: %s\n" % e)
[inline-code-end]