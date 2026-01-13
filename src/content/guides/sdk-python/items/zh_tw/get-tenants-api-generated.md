## 參數

| 名稱 | 類型 | 位置 | 必填 | 說明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| meta | string | query | No |  |
| skip | number | query | No |  |

## 回應

回傳: [`GetTenants200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_tenants200_response.py)

## 範例

[inline-code-attrs-start title = 'get_tenants 範例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_tenants200_response import GetTenants200Response
from client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to https://fastcomments.com
# 請參閱 configuration.py 以取得所有支援的設定參數清單。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# The client must configure the authentication and authorization parameters
# 根據 API 伺服器的安全性策略，客戶端必須設定驗證與授權參數。
# Examples for each auth method are provided below, use the example that
# 下面提供每種驗證方法的範例，請使用符合您驗證需求的範例。

# Configure API key authorization: api_key
# 設定 API 金鑰授權：api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
# 如有需要，取消註解下方以為 API 金鑰設定前綴（例如 Bearer）

# Enter a context with an instance of the API client
# 使用 API client 的實例進入一個上下文
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    # 建立 API 類別的實例
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    meta = 'meta_example' # str |  （選用）
    skip = 3.4 # float |  （選用）

    try:
        api_response = api_instance.get_tenants(tenant_id, meta=meta, skip=skip)
        print("The response of DefaultApi->get_tenants:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_tenants: %s\n" % e)
[inline-code-end]