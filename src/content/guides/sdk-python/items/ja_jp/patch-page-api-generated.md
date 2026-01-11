## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| id | string | path | はい |  |

## レスポンス

戻り値: [`PatchPageAPIResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/patch_page_api_response.py)

## 例

[inline-code-attrs-start title = 'patch_page の例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.patch_page_api_response import PatchPageAPIResponse
from client.models.update_api_page_data import UpdateAPIPageData
from client.rest import ApiException
from pprint import pprint

# ホストの定義はオプションで、デフォルトは https://fastcomments.com です
# サポートされているすべての設定パラメータの一覧は configuration.py を参照してください。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# クライアントは API サーバーのセキュリティポリシーに従って認証および認可パラメータを設定する必要があります。
# 各認証方式の例を以下に示します。自分の認証ケースに合う例を使用してください。

# APIキー認証を設定する: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 必要に応じて API キーのプレフィックス（例: Bearer）を設定するには、以下のコメントを解除してください
# configuration.api_key_prefix['api_key'] = 'Bearer'

# API クライアントのインスタンスとともにコンテキストに入ります
with client.ApiClient(configuration) as api_client:
    # API クラスのインスタンスを作成します
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    update_api_page_data = client.UpdateAPIPageData() # UpdateAPIPageData | 

    try:
        api_response = api_instance.patch_page(tenant_id, id, update_api_page_data)
        print("The response of DefaultApi->patch_page:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->patch_page: %s\n" % e)
[inline-code-end]

---