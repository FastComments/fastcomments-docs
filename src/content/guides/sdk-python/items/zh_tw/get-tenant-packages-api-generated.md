## 參數

| 名稱 | 類型 | 位置 | 是否必填 | 說明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| skip | number | query | 否 |  |

## 回應

回傳: [`GetTenantPackagesResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_tenant_packages_response.py)

## 範例

[inline-code-attrs-start title = 'get_tenant_packages 範例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_tenant_packages_response import GetTenantPackagesResponse
from client.rest import ApiException
from pprint import pprint

# 定義 host 為選用，預設為 https://fastcomments.com
# 請參見 configuration.py 以取得所有支援的設定參數清單。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# 用戶端必須依 API 伺服器的安全政策設定驗證與授權參數。
# 下方提供各驗證方法的範例，請使用符合您驗證需求的範例。

# 設定 API 金鑰授權：api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 若需要，解除註解下列程式以設定 API 金鑰的前綴（例如 Bearer）
# configuration.api_key_prefix['api_key'] = 'Bearer'

# 使用 ApiClient 的實例進入一個上下文
with client.ApiClient(configuration) as api_client:
    # 建立 API 類別的實例
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    skip = 3.4 # float |  (可選)

    try:
        api_response = api_instance.get_tenant_packages(tenant_id, skip=skip)
        print("The response of DefaultApi->get_tenant_packages:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_tenant_packages: %s\n" % e)
[inline-code-end]