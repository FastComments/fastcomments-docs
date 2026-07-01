## Parameters

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | path | はい |  |
| postIds | array | query | いいえ |  |
| sso | string | query | いいえ |  |

## Response

戻り値: [`UserReactsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/user_reacts_response.py)

## Example

[inline-code-attrs-start title = 'get_user_reacts_public 例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import GetUserReactsPublicOptions
from client.models.user_reacts_response import UserReactsResponse
from client.rest import ApiException
from pprint import pprint

# ホストの定義はオプションで、デフォルトは https://fastcomments.com です
# configuration.py でサポートされているすべての設定パラメータの一覧を確認してください。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# APIクライアントのインスタンスでコンテキストに入ります
with client.ApiClient(configuration) as api_client:
    # APIクラスのインスタンスを作成します
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    post_ids = ['post_ids_example'] # List[str] |  (optional)
    sso = 'sso_example' # str |  (optional)

    try:
        api_response = api_instance.get_user_reacts_public(tenant_id, GetUserReactsPublicOptions(post_ids=post_ids, sso=sso))
        print("PublicApi->get_user_reacts_public のレスポンス:\n")
        pprint(api_response)
    except Exception as e:
        print("PublicApi->get_user_reacts_public を呼び出す際の例外: %s\n" % e)
[inline-code-end]