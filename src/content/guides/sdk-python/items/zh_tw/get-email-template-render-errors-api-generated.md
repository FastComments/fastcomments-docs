## 參數

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| id | string | path | 是 |  |
| skip | number | query | 否 |  |

## 回傳

Returns: [`GetEmailTemplateRenderErrorsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_email_template_render_errors_response.py)

## 範例

[inline-code-attrs-start title = 'get_email_template_render_errors 範例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_email_template_render_errors_response import GetEmailTemplateRenderErrorsResponse
from client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to https://fastcomments.com
# 有關全部支援的設定參數列表，請見 configuration.py。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# The client must configure the authentication and authorization parameters
# in accordance with the API server security policy.
# Examples for each auth method are provided below, use the example that
# satisfies your auth use case.
# 用戶端必須根據 API 伺服器的安全政策設定驗證與授權參數。
# 下方提供了每種授權方法的範例，請使用符合您授權情境的範例。

# Configure API key authorization: api_key
# 設定 API 金鑰授權：api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
# 若需要，取消註解下方以為 API 金鑰設定前綴（例如 Bearer）

# Enter a context with an instance of the API client
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    # 建立 API 類別的實例
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    skip = 3.4 # float |  (可選)

    try:
        api_response = api_instance.get_email_template_render_errors(tenant_id, id, skip=skip)
        print("The response of DefaultApi->get_email_template_render_errors:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_email_template_render_errors: %s\n" % e)
[inline-code-end]