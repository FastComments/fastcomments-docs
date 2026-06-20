---
## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| id | string | path | はい |  |

## レスポンス

戻り値: [`APIEmptySuccessResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_empty_success_response.py)

## 例

[inline-code-attrs-start title = 'delete_user_badge の例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.api_empty_success_response import APIEmptySuccessResponse
from client.rest import ApiException
from pprint import pprint

# ホストの定義は任意で、省略するとデフォルトは https://fastcomments.com になります
# サポートされている設定パラメータの一覧は configuration.py を参照してください。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# クライアントは認証および認可のパラメータを設定する必要があります
# API サーバーのセキュリティポリシーに従って設定してください。
# 各認証方法の例を以下に示します。
# ご自身の認証ユースケースに合った例を使用してください。

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 必要に応じて API キーにプレフィックス（例: Bearer）を設定するには下の行のコメントを解除してください
# configuration.api_key_prefix['api_key'] = 'Bearer'

# API クライアントのインスタンスを使用してコンテキストに入ります
with client.ApiClient(configuration) as api_client:
    # API クラスのインスタンスを作成します
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 

    try:
        api_response = api_instance.delete_user_badge(tenant_id, id)
        print("The response of DefaultApi->delete_user_badge:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->delete_user_badge: %s\n" % e)
[inline-code-end]

---