## パラメータ

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| page | integer | query | いいえ |  |
| limit | integer | query | いいえ |  |
| skip | integer | query | いいえ |  |
| asTree | boolean | query | いいえ |  |
| skipChildren | integer | query | いいえ |  |
| limitChildren | integer | query | いいえ |  |
| maxTreeDepth | integer | query | いいえ |  |
| urlId | string | query | いいえ |  |
| userId | string | query | いいえ |  |
| anonUserId | string | query | いいえ |  |
| contextUserId | string | query | いいえ |  |
| hashTag | string | query | いいえ |  |
| parentId | string | query | いいえ |  |
| direction | string | query | いいえ |  |
| fromDate | integer | query | いいえ |  |
| toDate | integer | query | いいえ |  |

## レスポンス

戻り値: [`APIGetCommentsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_get_comments_response.py)

## 例

[inline-code-attrs-start title = 'get_comments の例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.api_get_comments_response import APIGetCommentsResponse
from client.models.sort_directions import SortDirections
from client.rest import ApiException
from pprint import pprint

# ホストの定義は任意で、デフォルトは https://fastcomments.com です
# サポートされているすべての設定パラメータの一覧は configuration.py を参照してください。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# クライアントは、API サーバーのセキュリティポリシーに従って認証および認可のパラメータを設定する必要があります。
# 各認証方法の例を以下に示します。自身の認証ユースケースに合う例を使用してください。

# API キー認証を設定します: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 必要に応じて API キーのプレフィックス（例: Bearer）を設定するには、以下のコメントを外してください
# configuration.api_key_prefix['api_key'] = 'Bearer'

# API クライアントのインスタンスのコンテキストに入ります
with client.ApiClient(configuration) as api_client:
    # API クラスのインスタンスを作成します
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    page = 56 # int |  （オプション）
    limit = 56 # int |  （オプション）
    skip = 56 # int |  （オプション）
    as_tree = True # bool |  （オプション）
    skip_children = 56 # int |  （オプション）
    limit_children = 56 # int |  （オプション）
    max_tree_depth = 56 # int |  （オプション）
    url_id = 'url_id_example' # str |  （オプション）
    user_id = 'user_id_example' # str |  （オプション）
    anon_user_id = 'anon_user_id_example' # str |  （オプション）
    context_user_id = 'context_user_id_example' # str |  （オプション）
    hash_tag = 'hash_tag_example' # str |  （オプション）
    parent_id = 'parent_id_example' # str |  （オプション）
    direction = client.SortDirections() # SortDirections |  （オプション）
    from_date = 56 # int |  （オプション）
    to_date = 56 # int |  （オプション）

    try:
        api_response = api_instance.get_comments(tenant_id, page=page, limit=limit, skip=skip, as_tree=as_tree, skip_children=skip_children, limit_children=limit_children, max_tree_depth=max_tree_depth, url_id=url_id, user_id=user_id, anon_user_id=anon_user_id, context_user_id=context_user_id, hash_tag=hash_tag, parent_id=parent_id, direction=direction, from_date=from_date, to_date=to_date)
        print("The response of DefaultApi->get_comments:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_comments: %s\n" % e)
[inline-code-end]