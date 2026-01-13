## パラメータ

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| meta | string | query | いいえ |  |
| skip | number | query | いいえ |  |

## レスポンス

戻り値: [`GetTenants200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_tenants200_response.py)

## 例

[inline-code-attrs-start title = 'get_tenants の例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_tenants200_response import GetTenants200Response
from client.rest import ApiException
from pprint import pprint

# ホストの定義は任意で、デフォルトは https://fastcomments.com です
# サポートされているすべての設定パラメータの一覧は configuration.py を参照してください。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# クライアントは認証および認可のパラメータを設定する必要があります
# API サーバのセキュリティポリシーに従ってください。
# 以下に各認証方式の例を示します。
# ご自身の認証ユースケースに合う例を使用してください。

# API キー認証の設定: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 必要に応じて API キーの接頭辞（例: Bearer）を設定するには、下のコメントを解除してください
# configuration.api_key_prefix['api_key'] = 'Bearer'

# API クライアントのインスタンスを使ってコンテキストに入ります
with client.ApiClient(configuration) as api_client:
    # API クラスのインスタンスを作成します
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    meta = 'meta_example' # str |  （省略可）
    skip = 3.4 # float |  （省略可）

    try:
        api_response = api_instance.get_tenants(tenant_id, meta=meta, skip=skip)
        print("The response of DefaultApi->get_tenants:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_tenants: %s\n" % e)
[inline-code-end]