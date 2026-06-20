## 參數

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| id | string | path | 是 |  |

## 回應

回傳: [`GetModeratorResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_moderator_response.py)

## 範例

[inline-code-attrs-start title = 'get_moderator 範例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_moderator_response import GetModeratorResponse
from client.rest import ApiException
from pprint import pprint

# 定義主機為選用且預設為 https://fastcomments.com
# 請參閱 configuration.py 以取得所有支援的設定參數清單。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# 用戶端必須設定身分驗證與授權參數
# 以符合 API 伺服器的安全政策。
# 以下在下方提供每種驗證方法的範例，請使用能
# 滿足您驗證使用情境的範例。
# 設定 API 金鑰授權: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 如需，請取消註解下方以設定 API 金鑰的前綴（例如 Bearer）
# Enter a context with an instance of the API client
with client.ApiClient(configuration) as api_client:
    # 建立 API 類別的實例
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 

    try:
        api_response = api_instance.get_moderator(tenant_id, id)
        print("The response of DefaultApi->get_moderator:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_moderator: %s\n" % e)
[inline-code-end]