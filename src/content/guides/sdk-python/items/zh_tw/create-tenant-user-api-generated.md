## 參數

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |

## 回應

返回: [`CreateTenantUser200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/create_tenant_user200_response.py)

## 範例

[inline-code-attrs-start title = 'create_tenant_user 範例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.create_tenant_user200_response import CreateTenantUser200Response
from client.models.create_tenant_user_body import CreateTenantUserBody
from client.rest import ApiException
from pprint import pprint

# 定義 host 為選用，預設為 https://fastcomments.com
# 請參閱 configuration.py 以取得所有支援的設定參數清單。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# 用戶端必須根據 API 伺服器的安全性政策設定驗證與授權參數。
# 以下提供每種驗證方法的範例，請使用符合您驗證使用情境的範例。

# 設定 API 金鑰授權：api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 如有需要，取消註解下列以為 API 金鑰設定前綴（例如 Bearer）
# configuration.api_key_prefix['api_key'] = 'Bearer'

# 使用 API client 實例進入上下文
with client.ApiClient(configuration) as api_client:
    # 建立 API 類別的實例
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    create_tenant_user_body = client.CreateTenantUserBody() # CreateTenantUserBody | 

    try:
        api_response = api_instance.create_tenant_user(tenant_id, create_tenant_user_body)
        print("The response of DefaultApi->create_tenant_user:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->create_tenant_user: %s\n" % e)
[inline-code-end]