---
## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| id | string | path | はい |  |

## レスポンス

戻り値: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/flag_comment_public200_response.py)

## 例

[inline-code-attrs-start title = 'delete_pending_webhook_event の例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.flag_comment_public200_response import FlagCommentPublic200Response
from client.rest import ApiException
from pprint import pprint

# ホストの定義は任意で、デフォルトは https://fastcomments.com です
# サポートされているすべての設定パラメータの一覧は configuration.py を参照してください。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# クライアントは認証と認可のパラメータを設定する必要があります
# API サーバーのセキュリティポリシーに従ってください。
# 以下に各認証方式の例を示します。自分のユースケースに合う例を使用してください。

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 必要に応じて下のコメントを外して API キーのプレフィックス（例: Bearer）を設定してください
# configuration.api_key_prefix['api_key'] = 'Bearer'

# API クライアントのインスタンスを使ってコンテキストに入ります
with client.ApiClient(configuration) as api_client:
    # API クラスのインスタンスを作成します
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 

    try:
        api_response = api_instance.delete_pending_webhook_event(tenant_id, id)
        print("The response of DefaultApi->delete_pending_webhook_event:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->delete_pending_webhook_event: %s\n" % e)
[inline-code-end]

---