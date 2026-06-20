## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| userId | string | query | No |  |
| state | number | query | No |  |
| skip | number | query | No |  |
| limit | number | query | No |  |

## Response

戻り値: [`GetTicketsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_tickets_response.py)

## Example

[inline-code-attrs-start title = 'get_tickets の例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_tickets_response import GetTicketsResponse
from client.rest import ApiException
from pprint import pprint

# ホストの定義は省略可能で、デフォルトは https://fastcomments.com です
# サポートされているすべての設定パラメータの一覧は configuration.py を参照してください。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# クライアントは認証および認可パラメータを設定する必要があります
# API サーバのセキュリティポリシーに従ってください。
# 各認証方式の例を以下に示します。
# ご利用の認証方法に合う例を使用してください。

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# コメントアウトを外して、必要に応じて API キーにプレフィックス（例: Bearer）を設定してください
# configuration.api_key_prefix['api_key'] = 'Bearer'

# API クライアントのインスタンスを用いてコンテキストに入ります
with client.ApiClient(configuration) as api_client:
    # API クラスのインスタンスを作成
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    user_id = 'user_id_example' # str |  (optional)
    state = 3.4 # float |  (optional)
    skip = 3.4 # float |  (optional)
    limit = 3.4 # float |  (optional)

    try:
        api_response = api_instance.get_tickets(tenant_id, user_id=user_id, state=state, skip=skip, limit=limit)
        print("The response of DefaultApi->get_tickets:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_tickets: %s\n" % e)
[inline-code-end]