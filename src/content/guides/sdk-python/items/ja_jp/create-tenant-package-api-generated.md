## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |

## レスポンス

戻り値: [`CreateTenantPackageResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/create_tenant_package_response.py)

## 例

[inline-code-attrs-start title = 'create_tenant_package の例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.create_tenant_package_body import CreateTenantPackageBody
from client.models.create_tenant_package_response import CreateTenantPackageResponse
from client.rest import ApiException
from pprint import pprint

# ホストの定義は任意で、デフォルトは https://fastcomments.com です
# 使用可能な設定パラメータの一覧は configuration.py を参照してください。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# クライアントは認証および認可のパラメータを設定する必要があります
# これは API サーバーのセキュリティポリシーに従って行ってください。
# 以下に各認証方式の例を示します。 
# 自分の認証ユースケースに合う例を使用してください。

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Enter a context with an instance of the API client
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    create_tenant_package_body = client.CreateTenantPackageBody() # CreateTenantPackageBody | 

    try:
        api_response = api_instance.create_tenant_package(tenant_id, create_tenant_package_body)
        print("The response of DefaultApi->create_tenant_package:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->create_tenant_package: %s\n" % e)
[inline-code-end]