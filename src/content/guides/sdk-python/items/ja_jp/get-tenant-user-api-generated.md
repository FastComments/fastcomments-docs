---
## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| id | string | path | はい |  |

## レスポンス

戻り値: [`GetTenantUserResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_tenant_user_response.py)

## 例

[inline-code-attrs-start title = 'get_tenant_user の例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_tenant_user_response import GetTenantUserResponse
from client.rest import ApiException
from pprint import pprint

# ホストの定義は任意で、デフォルトは https://fastcomments.com です
# サポートされているすべての構成パラメータの一覧は configuration.py を参照してください。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# クライアントは API サーバーのセキュリティポリシーに従って認証および認可のパラメータを設定する必要があります。
# 各認証方法の例を下に示します。自分の認証ユースケースに合った例を使用してください。

# API キー認証を設定する: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 必要に応じて API キー用のプレフィックス（例: Bearer）を設定するには以下のコメントを解除してください
# configuration.api_key_prefix['api_key'] = 'Bearer'

# API クライアントのインスタンスでコンテキストに入る
with client.ApiClient(configuration) as api_client:
    # API クラスのインスタンスを作成する
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 

    try:
        api_response = api_instance.get_tenant_user(tenant_id, id)
        print("The response of DefaultApi->get_tenant_user:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_tenant_user: %s\n" % e)
[inline-code-end]

---