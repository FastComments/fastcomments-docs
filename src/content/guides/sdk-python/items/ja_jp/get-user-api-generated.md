## パラメータ

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |

## レスポンス

戻り値: [`GetUserResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_user_response.py)

## 例

[inline-code-attrs-start title = 'get_user の例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_user_response import GetUserResponse
from client.rest import ApiException
from pprint import pprint

# ホストの定義は省略可能で、デフォルトは https://fastcomments.com です
# サポートされている設定パラメータの一覧は configuration.py を参照してください。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# クライアントは認証および認可パラメータを設定する必要があります
# API サーバーのセキュリティポリシーに従ってください。
# 以下に各認証方式の例を示します。
# ご自身の認証ユースケースに合う例を使用してください。

# API キー認証を設定: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 必要に応じて以下のコメントアウトを外して、API キーのプレフィックス（例: Bearer）を設定します
# configuration.api_key_prefix['api_key'] = 'Bearer'

# API クライアントのインスタンスを使用するコンテキストに入ります
with client.ApiClient(configuration) as api_client:
    # API クラスのインスタンスを作成します
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 

    try:
        api_response = api_instance.get_user(tenant_id, id)
        print("The response of DefaultApi->get_user:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_user: %s\n" % e)
[inline-code-end]