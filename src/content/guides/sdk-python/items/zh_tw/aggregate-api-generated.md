聚合文件，透過分組（如果提供 groupBy）並套用多個運算。支援不同的運算（例如 sum、countDistinct、avg 等）。

## 參數

| 名稱 | 類型 | 位置 | 必填 | 說明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| parentTenantId | string | query | 否 |  |
| includeStats | boolean | query | 否 |  |

## 回應

回傳：[`AggregateResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/aggregate_response.py)

## 範例

[inline-code-attrs-start title = 'aggregate 範例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.aggregate_response import AggregateResponse
from client.models.aggregation_request import AggregationRequest
from client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to https://fastcomments.com
# 有關所有支援的設定參數清單，請參閱 configuration.py。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# The client must configure the authentication and authorization parameters
# in accordance with the API server security policy.
# Examples for each auth method are provided below, use the example that
# satisfies your auth use case.
# 用戶端必須依據 API 伺服器的安全性政策來設定驗證與授權參數。
# 下方提供各種驗證方法的範例，請使用符合您驗證場景的範例。

# Configure API key authorization: api_key
# 設定 API key 授權：api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
# 若需要，取消註解下方以設定 API key 的前綴（例如 Bearer）
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Enter a context with an instance of the API client
# 使用 API client 實例進入一個上下文
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    # 建立 API 類別的實例
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    aggregation_request = client.AggregationRequest() # AggregationRequest | 
    parent_tenant_id = 'parent_tenant_id_example' # str |  (可選)
    include_stats = True # bool |  (可選)

    try:
        api_response = api_instance.aggregate(tenant_id, aggregation_request, parent_tenant_id=parent_tenant_id, include_stats=include_stats)
        print("The response of DefaultApi->aggregate:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->aggregate: %s\n" % e)
[inline-code-end]