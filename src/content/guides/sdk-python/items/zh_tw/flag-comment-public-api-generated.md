## 參數

| 名稱 | 類型 | 位置 | 必填 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| commentId | string | path | Yes |  |
| isFlagged | boolean | query | Yes |  |
| sso | string | query | No |  |

## 回應

回傳: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/flag_comment_public200_response.py)

## 範例

[inline-code-attrs-start title = 'flag_comment_public 範例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.flag_comment_public200_response import FlagCommentPublic200Response
from client.rest import ApiException
from pprint import pprint

# 設定主機為選用，預設為 https://fastcomments.com
# 請參閱 configuration.py 以取得所有支援的配置參數清單。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# 以 API 客戶端實例進入上下文
with client.ApiClient(configuration) as api_client:
    # 建立 API 類別的實例
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    comment_id = 'comment_id_example' # str | 
    is_flagged = True # bool | 
    sso = 'sso_example' # str |  (選用)

    try:
        api_response = api_instance.flag_comment_public(tenant_id, comment_id, is_flagged, sso=sso)
        print("The response of PublicApi->flag_comment_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->flag_comment_public: %s\n" % e)
[inline-code-end]