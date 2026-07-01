## Parameters

| 名前 | タイプ | 場所 | 必須 | 説明 |
|------|------|----------|------|-------------|
| tenantId | string | query | はい |  |
| meta | string | query | いいえ |  |
| skip | number | query | いいえ |  |

## Response

返り値: [`GetTenantsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_tenants_response.py)

## Example

[inline-code-attrs-start title = 'get_tenants の例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.default_api import GetTenantsOptions
from client.models.get_tenants_response import GetTenantsResponse
from client.rest import ApiException
from pprint import pprint

# ホストの定義は任意で、デフォルトは https://fastcomments.com です
# configuration.py でサポートされているすべての設定パラメータの一覧を確認できます
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# クライアントは認証および認可パラメータを設定する必要があります
# APIサーバーのセキュリティポリシーに従って。
# 各認証方法の例が以下に提供されています。
# 使用ケースに合う例を使用してください。

# APIキー認証を設定します: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 必要に応じて、APIキーのプレフィックス（例: Bearer）を設定する場合は以下のコメントを外してください
# configuration.api_key_prefix['api_key'] = 'Bearer'

# APIクライアントのインスタンスでコンテキストに入ります
with client.ApiClient(configuration) as api_client:
    # APIクラスのインスタンスを作成します
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    meta = 'meta_example' # str |  (optional)
    skip = 3.4 # float |  (optional)

    try:
        api_response = api_instance.get_tenants(tenant_id, GetTenantsOptions(meta=meta, skip=skip))
        print("The response of DefaultApi->get_tenants:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_tenants: %s\n" % e)
[inline-code-end]