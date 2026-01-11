## 參數

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |

## 回應

回傳: [`AddSSOUserAPIResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/add_sso_user_api_response.py)

## 範例

[inline-code-attrs-start title = 'add_sso_user 範例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.add_sso_user_api_response import AddSSOUserAPIResponse
from client.models.create_apisso_user_data import CreateAPISSOUserData
from client.rest import ApiException
from pprint import pprint

# 設定 host 為選用，預設為 https://fastcomments.com
# 請參閱 configuration.py 以取得所有支援的設定參數清單。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# 用戶端必須依據 API 伺服器的安全政策設定身份驗證與授權參數。
# 下方提供各種驗證方法的範例，請使用符合您驗證需求的範例。

# 設定 API key 授權：api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 若需要，取消註解下列以設定 API key 的前綴（例如 Bearer）
# configuration.api_key_prefix['api_key'] = 'Bearer'

# 以 API client 實例開啟一個上下文
with client.ApiClient(configuration) as api_client:
    # 建立 API 類別的實例
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    create_apisso_user_data = client.CreateAPISSOUserData() # CreateAPISSOUserData | 

    try:
        api_response = api_instance.add_sso_user(tenant_id, create_apisso_user_data)
        print("The response of DefaultApi->add_sso_user:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->add_sso_user: %s\n" % e)
[inline-code-end]