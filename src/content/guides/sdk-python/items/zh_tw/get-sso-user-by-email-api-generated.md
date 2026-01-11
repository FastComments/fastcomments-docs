## 參數

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| email | string | path | 是 |  |

## 回應

返回: [`GetSSOUserByEmailAPIResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_sso_user_by_email_api_response.py)

## 範例

[inline-code-attrs-start title = 'get_sso_user_by_email 範例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_sso_user_by_email_api_response import GetSSOUserByEmailAPIResponse
from client.rest import ApiException
from pprint import pprint

# 定義 host 是可選的，預設為 https://fastcomments.com
# 請參閱 configuration.py 以取得所有支援的設定參數清單。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# 用戶端必須依據 API 伺服器的安全政策設定驗證與授權參數。
# 下方提供每種驗證方法的範例，請使用符合您驗證情境的範例。

# 設定 API 金鑰授權: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 如有需要，取消註解下列程式以為 API 金鑰設定前綴（例如 Bearer）
# configuration.api_key_prefix['api_key'] = 'Bearer'

# 使用 API client 的實例進入上下文
with client.ApiClient(configuration) as api_client:
    # 建立 API 類別的實例
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    email = 'email_example' # str | 

    try:
        api_response = api_instance.get_sso_user_by_email(tenant_id, email)
        print("The response of DefaultApi->get_sso_user_by_email:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_sso_user_by_email: %s\n" % e)
[inline-code-end]

---