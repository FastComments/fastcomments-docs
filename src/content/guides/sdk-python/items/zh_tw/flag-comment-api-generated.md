## 參數

| 名稱 | 類型 | 位置 | 必填 | 說明 |
|------|------|----------|------|-------------|
| tenantId | string | query | 是 |  |
| id | string | path | 是 |  |
| userId | string | query | 否 |  |
| anonUserId | string | query | 否 |  |

## 回應

返回：[`FlagCommentResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/flag_comment_response.py)

## 範例

[inline-code-attrs-start title = 'flag_comment 範例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.default_api import FlagCommentOptions
from client.models.flag_comment_response import FlagCommentResponse
from client.rest import ApiException
from pprint import pprint

# 定義主機是可選的，預設為 https://fastcomments.com
# 請參閱 configuration.py 以取得所有支援的設定參數清單。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# 客戶端必須依據 API 伺服器的安全政策設定驗證與授權參數。
# 以下提供每種驗證方法的範例，請使用符合您驗證需求的範例。

# 設定 API 金鑰授權: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 如需設定前綴（例如 Bearer）給 API 金鑰，請取消註解以下行
# configuration.api_key_prefix['api_key'] = 'Bearer'

# 使用 API 客戶端實例進入上下文
with client.ApiClient(configuration) as api_client:
    # 建立 API 類別的實例
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    user_id = 'user_id_example' # str |  (optional)
    anon_user_id = 'anon_user_id_example' # str |  (optional)

    try:
        api_response = api_instance.flag_comment(tenant_id, id, FlagCommentOptions(user_id=user_id, anon_user_id=anon_user_id))
        print("The response of DefaultApi->flag_comment:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->flag_comment: %s\n" % e)
[inline-code-end]