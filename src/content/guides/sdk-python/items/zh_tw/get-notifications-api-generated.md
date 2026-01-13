## 參數

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| userId | string | query | 否 |  |
| urlId | string | query | 否 |  |
| fromCommentId | string | query | 否 |  |
| viewed | boolean | query | 否 |  |
| type | string | query | 否 |  |
| skip | number | query | 否 |  |

## 回應

回傳：[`GetNotifications200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_notifications200_response.py)

## 範例

[inline-code-attrs-start title = 'get_notifications 範例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_notifications200_response import GetNotifications200Response
from client.rest import ApiException
from pprint import pprint

# 定義 host 是可選的，預設為 https://fastcomments.com
# 請參閱 configuration.py 以取得所有支援的設定參數清單。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# 用戶端必須依照 API 伺服器的安全性政策設定驗證與授權參數。
# 下方提供各種驗證方法的範例，請使用符合您驗證情境的範例。

# 設定 API key 驗證：api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 若需要，取消註解下列程式以設定 API key 的前綴 (例如 Bearer)
# configuration.api_key_prefix['api_key'] = 'Bearer'

# 以 API client 實例進入 context
with client.ApiClient(configuration) as api_client:
    # 建立 API 類別的實例
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    user_id = 'user_id_example' # str |  (可選)
    url_id = 'url_id_example' # str |  (可選)
    from_comment_id = 'from_comment_id_example' # str |  (可選)
    viewed = True # bool |  (可選)
    type = 'type_example' # str |  (可選)
    skip = 3.4 # float |  (可選)

    try:
        api_response = api_instance.get_notifications(tenant_id, user_id=user_id, url_id=url_id, from_comment_id=from_comment_id, viewed=viewed, type=type, skip=skip)
        print("The response of DefaultApi->get_notifications:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_notifications: %s\n" % e)
[inline-code-end]