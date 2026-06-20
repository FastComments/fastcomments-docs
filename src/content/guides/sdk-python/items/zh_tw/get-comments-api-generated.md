## 參數

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | 查詢 | 是 |  |
| page | integer | 查詢 | 否 |  |
| limit | integer | 查詢 | 否 |  |
| skip | integer | 查詢 | 否 |  |
| asTree | boolean | 查詢 | 否 |  |
| skipChildren | integer | 查詢 | 否 |  |
| limitChildren | integer | 查詢 | 否 |  |
| maxTreeDepth | integer | 查詢 | 否 |  |
| urlId | string | 查詢 | 否 |  |
| userId | string | 查詢 | 否 |  |
| anonUserId | string | 查詢 | 否 |  |
| contextUserId | string | 查詢 | 否 |  |
| hashTag | string | 查詢 | 否 |  |
| parentId | string | 查詢 | 否 |  |
| direction | string | 查詢 | 否 |  |
| fromDate | integer | 查詢 | 否 |  |
| toDate | integer | 查詢 | 否 |  |

## 回應

回傳: [`APIGetCommentsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_get_comments_response.py)

## 範例

[inline-code-attrs-start title = 'get_comments 範例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.api_get_comments_response import APIGetCommentsResponse
from client.models.sort_directions import SortDirections
from client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to https://fastcomments.com
# 請參閱 configuration.py 以查看所有支援的設定參數。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# The client must configure the authentication and authorization parameters
# 用戶端必須設定驗證與授權參數
# in accordance with the API server security policy.
# 以符合 API 伺服器的安全政策。
# Examples for each auth method are provided below, use the example that
# 下方提供每種驗證方法的範例，使用符合您驗證情境的範例。
# satisfies your auth use case.
# 滿足您的驗證使用情境。

# Configure API key authorization: api_key
# 設定 API key 授權：api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
# 若需要，取消註解下方以設定 API key 的前綴（例如 Bearer）
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Enter a context with an instance of the API client
# 使用 ApiClient 的實例進入一個上下文
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    # 建立 API 類別的實例
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    page = 56 # int |  （可選）
    limit = 56 # int |  （可選）
    skip = 56 # int |  （可選）
    as_tree = True # bool |  （可選）
    skip_children = 56 # int |  （可選）
    limit_children = 56 # int |  （可選）
    max_tree_depth = 56 # int |  （可選）
    url_id = 'url_id_example' # str |  （可選）
    user_id = 'user_id_example' # str |  （可選）
    anon_user_id = 'anon_user_id_example' # str |  （可選）
    context_user_id = 'context_user_id_example' # str |  （可選）
    hash_tag = 'hash_tag_example' # str |  （可選）
    parent_id = 'parent_id_example' # str |  （可選）
    direction = client.SortDirections() # SortDirections |  （可選）
    from_date = 56 # int |  （可選）
    to_date = 56 # int |  （可選）

    try:
        api_response = api_instance.get_comments(tenant_id, page=page, limit=limit, skip=skip, as_tree=as_tree, skip_children=skip_children, limit_children=limit_children, max_tree_depth=max_tree_depth, url_id=url_id, user_id=user_id, anon_user_id=anon_user_id, context_user_id=context_user_id, hash_tag=hash_tag, parent_id=parent_id, direction=direction, from_date=from_date, to_date=to_date)
        print("The response of DefaultApi->get_comments:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_comments: %s\n" % e)
[inline-code-end]