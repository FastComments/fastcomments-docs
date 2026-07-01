## 參數

| 名稱 | 類型 | 位置 | 必填 | 說明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| meta | string | query | 否 |  |
| skip | number | query | 否 |  |

## 回應

Returns: [`GetTenantsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_tenants_response.py)

## 範例

[inline-code-attrs-start title = 'get_tenants 範例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.default_api import GetTenantsOptions
from client.models.get_tenants_response import GetTenantsResponse
from client.rest import ApiException
from pprint import pprint

# 定義主機是可選的，預設為 https://fastcomments.com
# 參閱 configuration.py 以取得所有支援的設定參數清單。
# 用戶端必須依照 API 伺服器的安全政策設定驗證與授權參數
# 於符合 API 伺服器安全政策的情況下。
# 以下提供每種認證方法的範例，使用符合您認證使用情境的範例。

# 設定 API 金鑰授權：api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 取消註解以下以設定前綴（例如 Bearer）給 API 金鑰，如有需要
# configuration.api_key_prefix['api_key'] = 'Bearer'

# 進入包含 API 用戶端實例的上下文
with client.ApiClient(configuration) as api_client:
    # 建立 API 類別的實例
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    meta = 'meta_example' # str |  (optional)
    skip = 3.4 # float |  (optional)

    try:
        api_response = api_instance.get_tenants(tenant_id, GetTenantsOptions(meta=meta, skip=skip))
        print("The response of DefaultApi->get_tenants:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_tenants: %s\n" % e)
[inline-code-end]