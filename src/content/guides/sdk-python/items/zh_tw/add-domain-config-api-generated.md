## 參數

| 名稱 | 類型 | 位置 | 必填 | 說明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |

## 回應

回傳: [`AddDomainConfigResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/add_domain_config_response.py)

## 範例

[inline-code-attrs-start title = 'add_domain_config 範例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.add_domain_config_params import AddDomainConfigParams
from client.models.add_domain_config_response import AddDomainConfigResponse
from client.rest import ApiException
from pprint import pprint

# 定義 host 是可選的，預設為 https://fastcomments.com
# 請參閱 configuration.py 以取得所有支援的設定參數清單。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# 用戶端必須設定驗證和授權參數
# 以符合 API 伺服器的安全政策。
# 下方提供每種驗證方法的範例，
# 請使用符合您驗證案例的範例。

# 設定 API 金鑰授權: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 若需要，取消註解下面程式碼以為 API 金鑰設定前綴（例如 Bearer）
# configuration.api_key_prefix['api_key'] = 'Bearer'

# 在 API client 的實例上下文中
with client.ApiClient(configuration) as api_client:
    # 建立 API 類別的實例
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    add_domain_config_params = client.AddDomainConfigParams() # AddDomainConfigParams | 

    try:
        api_response = api_instance.add_domain_config(tenant_id, add_domain_config_params)
        print("The response of DefaultApi->add_domain_config:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->add_domain_config: %s\n" % e)
[inline-code-end]