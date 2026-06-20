## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| id | string | path | はい |  |

## レスポンス

戻り値: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_empty_response.py)

## 例

[inline-code-attrs-start title = 'update_tenant の例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.api_empty_response import APIEmptyResponse
from client.models.update_tenant_body import UpdateTenantBody
from client.rest import ApiException
from pprint import pprint

# ホストの定義は任意で、省略した場合のデフォルトは https://fastcomments.com です
# サポートされているすべての設定パラメータの一覧は configuration.py を参照してください。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# クライアントは API サーバーのセキュリティポリシーに従って認証および認可パラメータを設定する必要があります。
# 各認証方法の例を以下に示します。利用する認証方法に合わせて該当する例を使用してください。

# API キー認証を設定します: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 必要に応じて、API キーにプレフィックス（例: Bearer）を設定する場合は以下の行のコメントを外してください
# configuration.api_key_prefix['api_key'] = 'Bearer'

# API クライアントのインスタンスを使ってコンテキストに入ります
with client.ApiClient(configuration) as api_client:
    # API クラスのインスタンスを作成します
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    update_tenant_body = client.UpdateTenantBody() # UpdateTenantBody | 

    try:
        api_response = api_instance.update_tenant(tenant_id, id, update_tenant_body)
        print("The response of DefaultApi->update_tenant:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->update_tenant: %s\n" % e)
[inline-code-end]