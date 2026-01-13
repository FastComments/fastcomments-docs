## パラメータ

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| id | string | path | はい |  |

## レスポンス

戻り値: [`GetTenantUser200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_tenant_user200_response.py)

## 例

[inline-code-attrs-start title = 'get_tenant_user の例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_tenant_user200_response import GetTenantUser200Response
from client.rest import ApiException
from pprint import pprint

# ホストの定義は任意で、省略した場合は https://fastcomments.com がデフォルトになります
# サポートされているすべての設定パラメーターの一覧は configuration.py を参照してください。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# クライアントは認証および認可のパラメーターを設定する必要があります
# これは API サーバーのセキュリティポリシーに従って行ってください。
# 各認証方式の例を以下に示します。 
# ご利用の認証ケースに合う例を使用してください。

# API キー認証を設定します: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 必要に応じて、API キーのプレフィックス（例: Bearer）を設定するには以下のコメントを解除してください
# configuration.api_key_prefix['api_key'] = 'Bearer'

# API クライアントのインスタンスを用いてコンテキストを開始します
with client.ApiClient(configuration) as api_client:
    # API クラスのインスタンスを作成します
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