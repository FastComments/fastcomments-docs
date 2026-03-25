## パラメータ

| 名前 | 型 | 位置 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| userId | string | query | いいえ |  |
| state | number | query | いいえ |  |
| skip | number | query | いいえ |  |
| limit | number | query | いいえ |  |

## レスポンス

戻り値: [`GetTickets200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_tickets200_response.py)

## 例

[inline-code-attrs-start title = 'get_tickets の例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_tickets200_response import GetTickets200Response
from client.rest import ApiException
from pprint import pprint

# ホストの定義はオプションで、デフォルトは https://fastcomments.com です
# サポートされているすべての設定パラメータの一覧は configuration.py を参照してください。

# クライアントは認証および認可のパラメータを設定する必要があります
# API サーバーのセキュリティポリシーに従ってください。
# 各認証方式の例は以下に示されています。該当する例を
# ご利用の認証ユースケースに合わせて選んでください。

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Enter a context with an instance of the API client
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    user_id = 'user_id_example' # str |  (オプション)
    state = 3.4 # float |  (オプション)
    skip = 3.4 # float |  (オプション)
    limit = 3.4 # float |  (オプション)

    try:
        api_response = api_instance.get_tickets(tenant_id, user_id=user_id, state=state, skip=skip, limit=limit)
        print("The response of DefaultApi->get_tickets:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_tickets: %s\n" % e)
[inline-code-end]