## 參數

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| skip | integer | query | No |  |

## 回應

回傳: [`GetSSOUsersResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_sso_users_response.py)

## 範例

[inline-code-attrs-start title = 'get_sso_users 範例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_sso_users_response import GetSSOUsersResponse
from client.rest import ApiException
from pprint import pprint

# 定義 host 為選用，預設為 https://fastcomments.com
# 請參閱 configuration.py 以取得所有支援的設定參數清單。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# 用戶端必須設定驗證與授權參數
# 以符合 API 伺服器的安全政策。
# 下方提供每種驗證方法的範例，請使用符合您驗證情境的範例。

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 如有需要，取消註解下方以設定 API key 的前綴（例如 Bearer）
# configuration.api_key_prefix['api_key'] = 'Bearer'

# 以 API client 實例建立上下文
with client.ApiClient(configuration) as api_client:
    # 建立 API 類別的實例
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    skip = 56 # int |  (可選)

    try:
        api_response = api_instance.get_sso_users(tenant_id, skip=skip)
        print("The response of DefaultApi->get_sso_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_sso_users: %s\n" % e)
[inline-code-end]