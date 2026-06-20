## 參數

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | 查詢 | 是 |  |
| id | string | 路徑 | 是 |  |

## 回應

回傳: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_empty_response.py)

## 範例

[inline-code-attrs-start title = 'update_tenant 範例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.api_empty_response import APIEmptyResponse
from client.models.update_tenant_body import UpdateTenantBody
from client.rest import ApiException
from pprint import pprint

# 定義 host 為選用，預設為 https://fastcomments.com
# 請參閱 configuration.py 了解所有支援的設定參數清單。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# 用戶端必須依據 API 伺服器的安全政策設定身分驗證與授權參數。
# 下方提供各種認證方法的範例，請使用符合您認證需求的範例。

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 如需，取消註解下列程式以設定 API 金鑰的前綴（例如 Bearer）
# configuration.api_key_prefix['api_key'] = 'Bearer'

# 使用 ApiClient 實例進入執行環境
with client.ApiClient(configuration) as api_client:
    # 建立 API 類別的實例
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    update_tenant_body = client.UpdateTenantBody() # UpdateTenantBody | 

    try:
        api_response = api_instance.update_tenant(tenant_id, id, update_tenant_body)
        print("The response of DefaultApi->update_tenant:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->update_tenant: %s\n" % e)
[inline-code-end]