## 參數

| 名稱 | 類型 | 位置 | 必填 | 說明 |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| commentId | string | path | Yes |  |
| broadcastId | string | query | Yes |  |
| editKey | string | query | No |  |
| sso | string | query | No |  |

## 回應

回傳: [`PublicAPISetCommentTextResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/public_api_set_comment_text_response.py)

## 範例

[inline-code-attrs-start title = 'set_comment_text 範例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.comment_text_update_request import CommentTextUpdateRequest
from client.models.public_api_set_comment_text_response import PublicAPISetCommentTextResponse
from client.rest import ApiException
from pprint import pprint

# 定義 host 是選用的，預設為 https://fastcomments.com
# 請參閱 configuration.py 以取得所有支援的設定參數清單。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# 使用 API client 的實例開啟一個上下文
with client.ApiClient(configuration) as api_client:
    # 建立 API 類別的實例
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    comment_id = 'comment_id_example' # str | 
    broadcast_id = 'broadcast_id_example' # str | 
    comment_text_update_request = client.CommentTextUpdateRequest() # CommentTextUpdateRequest | 
    edit_key = 'edit_key_example' # str |  (選用)
    sso = 'sso_example' # str |  (選用)

    try:
        api_response = api_instance.set_comment_text(tenant_id, comment_id, broadcast_id, comment_text_update_request, edit_key=edit_key, sso=sso)
        print("The response of PublicApi->set_comment_text:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->set_comment_text: %s\n" % e)
[inline-code-end]