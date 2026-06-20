## パラメータ

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| userId | string | query | はい |  |

## レスポンス

返却: [`CreateTicketResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/create_ticket_response.py)

## 例

[inline-code-attrs-start title = 'create_ticket の例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.create_ticket_body import CreateTicketBody
from client.models.create_ticket_response import CreateTicketResponse
from client.rest import ApiException
from pprint import pprint

# ホストの定義は任意で、デフォルトは https://fastcomments.com です
# サポートされているすべての設定パラメータの一覧は configuration.py を参照してください。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# クライアントは認証および認可のパラメータを設定する必要があります
# API サーバーのセキュリティポリシーに従って。
# 各認証方式の例を以下に示します。 
# 自分の認証ユースケースに合う例を使用してください。

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 必要に応じて、API キー用のプレフィックス（例: Bearer）を設定するために以下の行のコメントアウトを解除してください
# configuration.api_key_prefix['api_key'] = 'Bearer'

# API クライアントのインスタンスを用いたコンテキストに入ります
with client.ApiClient(configuration) as api_client:
    # API クラスのインスタンスを作成します
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    user_id = 'user_id_example' # str | 
    create_ticket_body = client.CreateTicketBody() # CreateTicketBody | 

    try:
        api_response = api_instance.create_ticket(tenant_id, user_id, create_ticket_body)
        print("The response of DefaultApi->create_ticket:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->create_ticket: %s\n" % e)
[inline-code-end]