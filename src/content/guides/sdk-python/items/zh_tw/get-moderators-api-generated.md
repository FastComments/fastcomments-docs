## 參數

| 名稱 | 類型 | 位置 | 必填 | 說明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| skip | number | query | 否 |  |

## 回應

回傳: [`GetModerators200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_moderators200_response.py)

## 範例

[inline-code-attrs-start title = 'get_moderators 範例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_moderators200_response import GetModerators200Response
from client.rest import ApiException
from pprint import pprint

# 定義主機是可選的，預設為 https://fastcomments.com
# 有關所有支援的設定參數清單，請參閱 configuration.py。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# 用戶端必須根據 API 伺服器的安全政策
# 設定驗證與授權參數。
# 下方提供每種授權方法的範例，請使用能滿足您
# 的授權使用情境的範例。

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Enter a context with an instance of the API client
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    skip = 3.4 # float |  (optional)

    try:
        api_response = api_instance.get_moderators(tenant_id, skip=skip)
        print("The response of DefaultApi->get_moderators:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_moderators: %s\n" % e)
[inline-code-end]