---
## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| domain | string | path | はい |  |

## レスポンス

返却値: [`GetDomainConfigResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_domain_config_response.py)

## 例

[inline-code-attrs-start title = 'get_domain_config の例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_domain_config_response import GetDomainConfigResponse
from client.rest import ApiException
from pprint import pprint

# ホストの定義は任意で、デフォルトは https://fastcomments.com です
# サポートされているすべての設定パラメータの一覧は configuration.py を参照してください。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# クライアントは認証と認可のパラメータを構成する必要があります
# API サーバーのセキュリティポリシーに従って。
# 各認証方法の例は以下に示されています。使用する例は
# ご自身の認証ユースケースに合うものを使用してください。

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Enter a context with an instance of the API client
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    domain = 'domain_example' # str | 

    try:
        api_response = api_instance.get_domain_config(tenant_id, domain)
        print("The response of DefaultApi->get_domain_config:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_domain_config: %s\n" % e)
[inline-code-end]

---