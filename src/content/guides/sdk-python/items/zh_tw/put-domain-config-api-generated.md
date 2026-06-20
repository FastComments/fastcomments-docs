## 參數

| 名稱 | 類型 | 位置 | 必需 | 說明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| domainToUpdate | string | path | 是 |  |

## 回應

回傳: [`PutDomainConfigResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/put_domain_config_response.py)

## 範例

[inline-code-attrs-start title = 'put_domain_config 範例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.put_domain_config_response import PutDomainConfigResponse
from client.models.update_domain_config_params import UpdateDomainConfigParams
from client.rest import ApiException
from pprint import pprint

# 定義 host 是可選的，預設為 https://fastcomments.com
# 請參閱 configuration.py 以取得所有支援的設定參數清單。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# 用戶端必須設定認證與授權參數
# 以符合 API 伺服器的安全政策。
# 下方提供每種認證方法的範例，
# 請使用符合您認證使用情境的範例。

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
# configuration.api_key_prefix['api_key'] = 'Bearer'

# 使用 ApiClient 實例進入一個上下文
with client.ApiClient(configuration) as api_client:
    # 建立 API 類別的實例
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    domain_to_update = 'domain_to_update_example' # str | 
    update_domain_config_params = client.UpdateDomainConfigParams() # UpdateDomainConfigParams | 

    try:
        api_response = api_instance.put_domain_config(tenant_id, domain_to_update, update_domain_config_params)
        print("The response of DefaultApi->put_domain_config:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->put_domain_config: %s\n" % e)
[inline-code-end]