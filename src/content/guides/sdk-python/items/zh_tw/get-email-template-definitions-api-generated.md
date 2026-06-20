## 參數

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |

## 回應

回傳: [`GetEmailTemplateDefinitionsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_email_template_definitions_response.py)

## 範例

[inline-code-attrs-start title = 'get_email_template_definitions 範例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_email_template_definitions_response import GetEmailTemplateDefinitionsResponse
from client.rest import ApiException
from pprint import pprint

# 定義 host 為選用，預設為 https://fastcomments.com
# 請參閱 configuration.py 以取得所有支援的設定參數清單。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# 用戶端必須設定認證與授權參數
# 以符合 API 伺服器的安全性政策。
# 下方提供每種驗證方法的範例，請使用符合
# 您驗證使用情境的範例。

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
# 如有需要，取消註解下列內容以設定 API 金鑰的前綴（例如 Bearer）

# Enter a context with an instance of the API client
# 使用 API client 實例進入一個上下文
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    # 建立 API 類別的實例
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 

    try:
        api_response = api_instance.get_email_template_definitions(tenant_id)
        print("The response of DefaultApi->get_email_template_definitions:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_email_template_definitions: %s\n" % e)
[inline-code-end]