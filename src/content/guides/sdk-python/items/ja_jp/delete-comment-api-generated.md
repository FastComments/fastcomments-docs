## パラメータ

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| id | string | path | はい |  |
| contextUserId | string | query | いいえ |  |
| isLive | boolean | query | いいえ |  |

## レスポンス

返却: [`DeleteCommentResult`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/delete_comment_result.py)

## 例

[inline-code-attrs-start title = 'delete_comment の例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.delete_comment_result import DeleteCommentResult
from client.rest import ApiException
from pprint import pprint

# ホストの定義はオプションで、デフォルトは https://fastcomments.com です
# 使用可能な設定パラメータの一覧は configuration.py を参照してください。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# クライアントは認証および認可のパラメータを
# API サーバのセキュリティポリシーに従って設定する必要があります。
# 以下に各認証方式の例を示します。あなたのユースケースに合う例を使用してください。

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 必要に応じて API キーのプレフィックス（例: Bearer）を設定するには、以下の行のコメントを外してください
# configuration.api_key_prefix['api_key'] = 'Bearer'

# API クライアントのインスタンスでコンテキストに入ります
with client.ApiClient(configuration) as api_client:
    # API クラスのインスタンスを作成します
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    context_user_id = 'context_user_id_example' # str |  （オプション）
    is_live = True # bool |  （オプション）

    try:
        api_response = api_instance.delete_comment(tenant_id, id, context_user_id=context_user_id, is_live=is_live)
        print("The response of DefaultApi->delete_comment:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->delete_comment: %s\n" % e)
[inline-code-end]