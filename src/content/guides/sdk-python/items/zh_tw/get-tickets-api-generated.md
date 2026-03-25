## 參數

| 名稱 | 類型 | 位置 | 是否必填 | 說明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| userId | string | query | 否 |  |
| state | number | query | 否 |  |
| skip | number | query | 否 |  |
| limit | number | query | 否 |  |

## 回應

回傳: [`GetTickets200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_tickets200_response.py)

## 範例

[inline-code-attrs-start title = 'get_tickets 範例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_tickets200_response import GetTickets200Response
from client.rest import ApiException
from pprint import pprint

# 定義主機是可選的，預設為 https://fastcomments.com
# 請參閱 configuration.py 以取得所有支援的設定參數清單。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# 用戶端必須設定驗證與授權參數，
# 以符合 API 伺服器的安全性規範。
# 下面為每種驗證方法提供範例，請使用
# 適合您驗證情境的範例。

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 如需，取消下方註解以設定 API 金鑰的前綴（例如 Bearer）
# configuration.api_key_prefix['api_key'] = 'Bearer'

# 使用 API 用戶端實例進入上下文
with client.ApiClient(configuration) as api_client:
    # 建立 API 類別的實例
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    user_id = 'user_id_example' # str |  (選用)
    state = 3.4 # float |  (選用)
    skip = 3.4 # float |  (選用)
    limit = 3.4 # float |  (選用)

    try:
        api_response = api_instance.get_tickets(tenant_id, user_id=user_id, state=state, skip=skip, limit=limit)
        print("The response of DefaultApi->get_tickets:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_tickets: %s\n" % e)
[inline-code-end]

---