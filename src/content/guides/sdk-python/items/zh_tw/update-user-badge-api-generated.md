## 參數

| 名稱 | 類型 | 位置 | 必填 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | 查詢 | 是 |  |
| id | string | 路徑 | 是 |  |

## 回應

回傳: [`APIEmptySuccessResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_empty_success_response.py)

## 範例

[inline-code-attrs-start title = 'update_user_badge 範例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.api_empty_success_response import APIEmptySuccessResponse
from client.models.update_user_badge_params import UpdateUserBadgeParams
from client.rest import ApiException
from pprint import pprint

# 定義 host 為選用，預設為 https://fastcomments.com
# 請參閱 configuration.py 以取得所有支援的設定參數清單。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# 用戶端必須設定驗證與授權參數
# 以符合 API 伺服器的安全性政策。
# 下方提供每種授權方法的範例，請使用
# 符合您授權使用情境的範例。

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Enter a context with an instance of the API client
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    update_user_badge_params = client.UpdateUserBadgeParams() # UpdateUserBadgeParams | 

    try:
        api_response = api_instance.update_user_badge(tenant_id, id, update_user_badge_params)
        print("The response of DefaultApi->update_user_badge:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->update_user_badge: %s\n" % e)
[inline-code-end]