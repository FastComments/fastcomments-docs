## 参数

| Name | 类型 | 位置 | 必需 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| page | integer | query | 否 |  |
| limit | integer | query | 否 |  |
| skip | integer | query | 否 |  |
| asTree | boolean | query | 否 |  |
| skipChildren | integer | query | 否 |  |
| limitChildren | integer | query | 否 |  |
| maxTreeDepth | integer | query | 否 |  |
| urlId | string | query | 否 |  |
| userId | string | query | 否 |  |
| anonUserId | string | query | 否 |  |
| contextUserId | string | query | 否 |  |
| hashTag | string | query | 否 |  |
| parentId | string | query | 否 |  |
| direction | string | query | 否 |  |
| fromDate | integer | query | 否 |  |
| toDate | integer | query | 否 |  |

## 响应

返回: [`APIGetCommentsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_get_comments_response.py)

## 示例

[inline-code-attrs-start title = 'get_comments 示例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.api_get_comments_response import APIGetCommentsResponse
from client.models.sort_directions import SortDirections
from client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to https://fastcomments.com
# 请参阅 configuration.py 以获取所有受支持的配置参数列表。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# 客户端必须根据 API 服务器的安全策略配置认证和授权参数
# 并遵循 API 服务器的安全策略。
# 下文提供了每种认证方法的示例，请使用
# 满足您认证用例的示例。

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 如果需要，请取消注释下面行以为 API 密钥设置前缀（例如 Bearer）
# configuration.api_key_prefix['api_key'] = 'Bearer'

# 使用 API 客户端的一个实例进入上下文
with client.ApiClient(configuration) as api_client:
    # 创建 API 类的一个实例
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    page = 56 # int |  (可选)
    limit = 56 # int |  (可选)
    skip = 56 # int |  (可选)
    as_tree = True # bool |  (可选)
    skip_children = 56 # int |  (可选)
    limit_children = 56 # int |  (可选)
    max_tree_depth = 56 # int |  (可选)
    url_id = 'url_id_example' # str |  (可选)
    user_id = 'user_id_example' # str |  (可选)
    anon_user_id = 'anon_user_id_example' # str |  (可选)
    context_user_id = 'context_user_id_example' # str |  (可选)
    hash_tag = 'hash_tag_example' # str |  (可选)
    parent_id = 'parent_id_example' # str |  (可选)
    direction = client.SortDirections() # SortDirections |  (可选)
    from_date = 56 # int |  (可选)
    to_date = 56 # int |  (可选)

    try:
        api_response = api_instance.get_comments(tenant_id, page=page, limit=limit, skip=skip, as_tree=as_tree, skip_children=skip_children, limit_children=limit_children, max_tree_depth=max_tree_depth, url_id=url_id, user_id=user_id, anon_user_id=anon_user_id, context_user_id=context_user_id, hash_tag=hash_tag, parent_id=parent_id, direction=direction, from_date=from_date, to_date=to_date)
        print("The response of DefaultApi->get_comments:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_comments: %s\n" % e)
[inline-code-end]