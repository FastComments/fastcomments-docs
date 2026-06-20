## 參數

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| id | string | path | 是 |  |
| userId | string | query | 否 |  |

## 回應

回傳: [`GetTicketResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_ticket_response.py)

## 範例

[inline-code-attrs-start title = 'get_ticket 範例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_ticket_response import GetTicketResponse
from client.rest import ApiException
from pprint import pprint

# 設定 host 是可選的，預設為 https://fastcomments.com
# 請參閱 configuration.py 以取得所有支援的設定參數清單。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# 用戶端必須設定驗證與授權參數
# 以符合 API 伺服器的安全政策。
# 下方提供每種驗證方法的範例，請使用
# 符合您驗證使用情境的範例。

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 若需要，解除註解下面以設定 API 金鑰前綴（例如 Bearer）
# configuration.api_key_prefix['api_key'] = 'Bearer'

# 使用 API client 的實例進入一個上下文
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    user_id = 'user_id_example' # str |  (可選)

    try:
        api_response = api_instance.get_ticket(tenant_id, id, user_id=user_id)
        print("The response of DefaultApi->get_ticket:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_ticket: %s\n" % e)
[inline-code-end]