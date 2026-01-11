## パラメータ

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| id | string | path | はい |  |

## レスポンス

返却: [`UpdateUserBadge200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/update_user_badge200_response.py)

## 例

[inline-code-attrs-start title = 'update_user_badge の例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.update_user_badge200_response import UpdateUserBadge200Response
from client.models.update_user_badge_params import UpdateUserBadgeParams
from client.rest import ApiException
from pprint import pprint

# ホストの定義はオプションで、デフォルトは https://fastcomments.com です
# サポートされているすべての設定パラメータの一覧は configuration.py を参照してください。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# クライアントは認証および認可のパラメータを設定する必要があります
# API サーバーのセキュリティポリシーに従ってください。
# 各認証方式の例を以下に示します。 
# ご利用の認証ユースケースに合う例を使用してください。

# API キー認証を設定する: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 必要に応じて、API キーのプレフィックス（例: Bearer）を設定するには下のコメントアウトを解除してください
# configuration.api_key_prefix['api_key'] = 'Bearer'

# API クライアントのインスタンスでコンテキストに入ります
with client.ApiClient(configuration) as api_client:
    # API クラスのインスタンスを作成します
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    update_user_badge_params = client.UpdateUserBadgeParams() # UpdateUserBadgeParams | 

    try:
        api_response = api_instance.update_user_badge(tenant_id, id, update_user_badge_params)
        print("The response of DefaultApi->update_user_badge:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->update_user_badge: %s\n" % e)
[inline-code-end]