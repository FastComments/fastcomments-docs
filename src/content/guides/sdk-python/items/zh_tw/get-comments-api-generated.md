## 參數

| Name | Type | Location | Required | Description |
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

## 回應

回傳: [`GetComments200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_comments200_response.py)

## 範例

[inline-code-attrs-start title = 'get_comments 範例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_comments200_response import GetComments200Response
from client.models.sort_directions import SortDirections
from client.rest import ApiException
from pprint import pprint

# 定義主機為可選項，預設為 https://fastcomments.com
# 請參閱 configuration.py 以查看所有支援的設定參數。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# 用戶端必須根據 API 伺服器的安全策略設定驗證與授權參數。
# 下方提供每種驗證方法的範例，請使用符合您驗證案例的範例。

# 設定 API 金鑰授權：api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 如需設定 API 金鑰前綴（例如 Bearer），請解除下列註解
# configuration.api_key_prefix['api_key'] = 'Bearer'

# 使用 API 用戶端的實例進入上下文
with client.ApiClient(configuration) as api_client:
    # 建立 API 類別的實例
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    page = 56 # int |  (可選)
    limit = 56 # int |  (可選)
    skip = 56 # int |  (可選)
    as_tree = True # bool |  (可選)
    skip_children = 56 # int |  (可選)
    limit_children = 56 # int |  (可選)
    max_tree_depth = 56 # int |  (可選)
    url_id = 'url_id_example' # str |  (可選)
    user_id = 'user_id_example' # str |  (可選)
    anon_user_id = 'anon_user_id_example' # str |  (可選)
    context_user_id = 'context_user_id_example' # str |  (可選)
    hash_tag = 'hash_tag_example' # str |  (可選)
    parent_id = 'parent_id_example' # str |  (可選)
    direction = client.SortDirections() # SortDirections |  (可選)

    try:
        api_response = api_instance.get_comments(tenant_id, page=page, limit=limit, skip=skip, as_tree=as_tree, skip_children=skip_children, limit_children=limit_children, max_tree_depth=max_tree_depth, url_id=url_id, user_id=user_id, anon_user_id=anon_user_id, context_user_id=context_user_id, hash_tag=hash_tag, parent_id=parent_id, direction=direction)
        print("The response of DefaultApi->get_comments:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_comments: %s\n" % e)
[inline-code-end]

---