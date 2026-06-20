## 參數

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |
| contextUserId | string | query | No |  |
| doSpamCheck | boolean | query | No |  |
| isLive | boolean | query | No |  |

## 回應

回傳: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_empty_response.py)

## 範例

[inline-code-attrs-start title = 'update_comment 範例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.api_empty_response import APIEmptyResponse
from client.models.updatable_comment_params import UpdatableCommentParams
from client.rest import ApiException
from pprint import pprint

# 定義 host 是可選的，預設為 https://fastcomments.com
# 請參閱 configuration.py 以查看所有支援的設定參數清單。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# 用戶端必須依照 API 伺服器的安全政策設定驗證與授權參數。
# 下方提供各種驗證方法的範例，請使用能滿足您使用情境的範例。

# 設定 API 金鑰授權：api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 如需為 API 金鑰設定前綴（例如 Bearer），請取消註解下方程式碼
# configuration.api_key_prefix['api_key'] = 'Bearer'

# 使用 API 客戶端實例建立一個上下文
with client.ApiClient(configuration) as api_client:
    # 建立 API 類別的實例
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    updatable_comment_params = client.UpdatableCommentParams() # UpdatableCommentParams | 
    context_user_id = 'context_user_id_example' # str |  (可選)
    do_spam_check = True # bool |  (可選)
    is_live = True # bool |  (可選)

    try:
        api_response = api_instance.update_comment(tenant_id, id, updatable_comment_params, context_user_id=context_user_id, do_spam_check=do_spam_check, is_live=is_live)
        print("The response of DefaultApi->update_comment:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->update_comment: %s\n" % e)
[inline-code-end]

---