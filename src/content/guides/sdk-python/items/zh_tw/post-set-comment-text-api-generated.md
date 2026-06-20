---
## 參數

| 名稱 | 類型 | 位置 | 必填 | 描述 |
|------|------|----------|----------|-------------|
| commentId | string | path | 是 |  |
| sso | string | query | 否 |  |

## 回應

回傳: [`SetCommentTextResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/set_comment_text_response.py)

## 範例

[inline-code-attrs-start title = 'post_set_comment_text 範例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.set_comment_text_params import SetCommentTextParams
from client.models.set_comment_text_response import SetCommentTextResponse
from client.rest import ApiException
from pprint import pprint

# 定義 host 是可選的，預設為 https://fastcomments.com
# 有關所有支援的設定參數清單，請參閱 configuration.py。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# 透過 ApiClient 實例進入上下文
with client.ApiClient(configuration) as api_client:
    # 建立 API 類別的實例
    api_instance = client.ModerationApi(api_client)
    comment_id = 'comment_id_example' # str | 
    set_comment_text_params = client.SetCommentTextParams() # SetCommentTextParams | 
    sso = 'sso_example' # str |  (可選)

    try:
        api_response = api_instance.post_set_comment_text(comment_id, set_comment_text_params, sso=sso)
        print("The response of ModerationApi->post_set_comment_text:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->post_set_comment_text: %s\n" % e)
[inline-code-end]

---