## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| id | string | path | はい |  |
| userId | string | query | いいえ |  |
| anonUserId | string | query | いいえ |  |

## 応答

戻り値: [`FlagCommentResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/flag_comment_response.py)

## 例

[inline-code-attrs-start title = 'flag_comment の例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.default_api import FlagCommentOptions
from client.models.flag_comment_response import FlagCommentResponse
from client.rest import ApiException
from pprint import pprint

# ホストの定義はオプションで、デフォルトは https://fastcomments.com です
# configuration.py を参照して、サポートされているすべての設定パラメータの一覧を確認してください。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# クライアントは、認証および認可パラメータを設定する必要があります
# APIサーバーのセキュリティポリシーに従って
# 各認証方法の例が以下に提供されています。ご自分の認証ユースケースに合う例を使用してください。
# APIキー認証を設定: api_key
# 必要に応じて、APIキーのプレフィックス（例: Bearer）を設定する場合は、以下のコメントを解除してください
# configuration.api_key_prefix['api_key'] = 'Bearer'

# APIクライアントのインスタンスでコンテキストに入ります
with client.ApiClient(configuration) as api_client:
    # APIクラスのインスタンスを作成
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    user_id = 'user_id_example' # str |  （オプション）
    anon_user_id = 'anon_user_id_example' # str |  （オプション）

    try:
        api_response = api_instance.flag_comment(tenant_id, id, FlagCommentOptions(user_id=user_id, anon_user_id=anon_user_id))
        print("The response of DefaultApi->flag_comment:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->flag_comment: %s\n" % e)
[inline-code-end]