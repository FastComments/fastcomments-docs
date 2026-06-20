## 參數

| 名稱 | 類型 | 位置 | 必填 | 說明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| meta | string | query | 否 |  |
| skip | number | query | 否 |  |

## 回應

回傳: [`GetTenantsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_tenants_response.py)

## 範例

[inline-code-attrs-start title = 'get_tenants 範例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_tenants_response import GetTenantsResponse
from client.rest import ApiException
from pprint import pprint

# 定義 host 為選用，預設為 https://fastcomments.com
# 詳見 configuration.py 以取得所有支援的設定參數清單。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# 客戶端必須設定驗證與授權參數
# 以符合 API 伺服器的安全政策。
# 下方提供每種授權方法的範例，
# 請使用符合您授權使用情境的範例。

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
# configuration.api_key_prefix['api_key'] = 'Bearer'

# 使用 API 用戶端實例進入上下文
with client.ApiClient(configuration) as api_client:
    # 建立 API 類別的實例
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    meta = 'meta_example' # str |  (可選)
    skip = 3.4 # float |  (可選)

    try:
        api_response = api_instance.get_tenants(tenant_id, meta=meta, skip=skip)
        print("The response of DefaultApi->get_tenants:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_tenants: %s\n" % e)
[inline-code-end]