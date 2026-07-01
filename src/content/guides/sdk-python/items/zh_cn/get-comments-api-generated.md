## 参数

| 名称 | 类型 | 位置 | 必需 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| page | integer | query | No |  |
| limit | integer | query | No |  |
| skip | integer | query | No |  |
| asTree | boolean | query | No |  |
| skipChildren | integer | query | No |  |
| limitChildren | integer | query | No |  |
| maxTreeDepth | integer | query | No |  |
| urlId | string | query | No |  |
| userId | string | query | No |  |
| anonUserId | string | query | No |  |
| contextUserId | string | query | No |  |
| hashTag | string | query | No |  |
| parentId | string | query | No |  |
| direction | string | query | No |  |
| fromDate | integer | query | No |  |
| toDate | integer | query | No |  |

## 响应

返回：[`APIGetCommentsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_get_comments_response.py)

## 示例

[inline-code-attrs-start title = 'get_comments 示例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.default_api import GetCommentsOptions
from client.models.api_get_comments_response import APIGetCommentsResponse
from client.models.sort_directions import SortDirections
from client.rest import ApiException
from pprint import pprint

# 定义 host 是可选的，默认值为 https://fastcomments.com
# 查看 configuration.py 获取所有支持的配置参数列表。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# 客户端必须配置认证和授权参数
# 以符合 API 服务器的安全策略。
# 以下提供了每种认证方法的示例，请使用符合您认证用例的示例。
# 符合您的认证使用情况。

# 配置 API 密钥授权：api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 如有需要，取消注释以下行以设置 API 密钥的前缀（例如 Bearer）
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Enter a context with an instance of the API client
with client.ApiClient(configuration) as api_client:
    # 创建 API 类的实例
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    page = 56 # int |  （可选）
    limit = 56 # int |  （可选）
    skip = 56 # int |  （可选）
    as_tree = True # bool |  （可选）
    skip_children = 56 # int |  （可选）
    limit_children = 56 # int |  （可选）
    max_tree_depth = 56 # int |  （可选）
    url_id = 'url_id_example' # str |  （可选）
    user_id = 'user_id_example' # str |  （可选）
    anon_user_id = 'anon_user_id_example' # str |  （可选）
    context_user_id = 'context_user_id_example' # str |  （可选）
    hash_tag = 'hash_tag_example' # str |  （可选）
    parent_id = 'parent_id_example' # str |  （可选）
    direction = client.SortDirections() # SortDirections |  （可选）
    from_date = 56 # int |  （可选）
    to_date = 56 # int |  （可选）

    try:
        api_response = api_instance.get_comments(tenant_id, GetCommentsOptions(page=page, limit=limit, skip=skip, as_tree=as_tree, skip_children=skip_children, limit_children=limit_children, max_tree_depth=max_tree_depth, url_id=url_id, user_id=user_id, anon_user_id=anon_user_id, context_user_id=context_user_id, hash_tag=hash_tag, parent_id=parent_id, direction=direction, from_date=from_date, to_date=to_date))
        print("The response of DefaultApi->get_comments:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_comments: %s\n" % e)
[inline-code-end]