## 參數

| 名稱 | 類型 | 位置 | 必填 | 描述 |
|------|------|----------|------|-------------|
| tenantId | string | query | 是 |  |
| commentId | string | path | 是 |  |
| reviewed | boolean | query | 否 |  |
| broadcastId | string | query | 否 |  |
| sso | string | query | 否 |  |

## 回應

回傳: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_empty_response.py)

## 範例

[inline-code-attrs-start title = 'post_set_comment_review_status 範例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.moderation_api import PostSetCommentReviewStatusOptions
from client.models.api_empty_response import APIEmptyResponse
from client.rest import ApiException
from pprint import pprint

# 定義主機是可選的，預設為 https://fastcomments.com
# 請參閱 configuration.py 以取得所有支援的設定參數列表。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# 以 API 客戶端實例進入上下文
with client.ApiClient(configuration) as api_client:
    # 建立 API 類別的實例
    api_instance = client.ModerationApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    comment_id = 'comment_id_example' # str | 
    reviewed = True # bool |  (選用)
    broadcast_id = 'broadcast_id_example' # str |  (選用)
    sso = 'sso_example' # str |  (選用)

    try:
        api_response = api_instance.post_set_comment_review_status(tenant_id, comment_id, PostSetCommentReviewStatusOptions(reviewed=reviewed, broadcast_id=broadcast_id, sso=sso))
        print("The response of ModerationApi->post_set_comment_review_status:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->post_set_comment_review_status: %s\n" % e)
[inline-code-end]