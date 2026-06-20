## 參數

| 名稱 | 類型 | 位置 | 必填 | 說明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| domainToUpdate | string | path | 是 |  |

## 回應

回傳: [`PatchDomainConfigResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/patch_domain_config_response.py)

## 範例

[inline-code-attrs-start title = 'patch_domain_config 範例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.patch_domain_config_params import PatchDomainConfigParams
from client.models.patch_domain_config_response import PatchDomainConfigResponse
from client.rest import ApiException
from pprint import pprint

# 設定 host 為選項，預設為 https://fastcomments.com
# 請參考 configuration.py 以取得所有支援的設定參數。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# 用戶端必須依據 API 伺服器的安全政策設定認證與授權參數。
# 下方提供每種認證方法的範例，請使用符合您使用案例的範例。

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
# configuration.api_key_prefix['api_key'] = 'Bearer'

# 使用 ApiClient 的實例建立一個上下文
with client.ApiClient(configuration) as api_client:
    # 建立 API 類別的一個實例
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    domain_to_update = 'domain_to_update_example' # str | 
    patch_domain_config_params = client.PatchDomainConfigParams() # PatchDomainConfigParams | 

    try:
        api_response = api_instance.patch_domain_config(tenant_id, domain_to_update, patch_domain_config_params)
        print("The response of DefaultApi->patch_domain_config:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->patch_domain_config: %s\n" % e)
[inline-code-end]