## パラメータ

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| userId | string | query | いいえ |  |
| badgeId | string | query | いいえ |  |
| type | number | query | いいえ |  |
| displayedOnComments | boolean | query | いいえ |  |
| limit | number | query | いいえ |  |
| skip | number | query | いいえ |  |

## レスポンス

戻り値: [`APIGetUserBadgesResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_get_user_badges_response.py)

## 例

[inline-code-attrs-start title = 'get_user_badges の例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.api_get_user_badges_response import APIGetUserBadgesResponse
from client.rest import ApiException
from pprint import pprint

# ホストの定義は任意で、デフォルトは https://fastcomments.com です
# 対応しているすべての設定パラメータの一覧は configuration.py を参照してください。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# クライアントは API サーバーのセキュリティポリシーに従って認証および認可パラメータを設定する必要があります。
# 各認証方式の例を以下に示します。自分のユースケースに合う例を使用してください。

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 必要に応じて API キーのプレフィックス（例: Bearer）を設定するには、以下の行のコメントを解除してください。
# configuration.api_key_prefix['api_key'] = 'Bearer'

# API クライアントのインスタンスを使ってコンテキストに入ります
with client.ApiClient(configuration) as api_client:
    # API クラスのインスタンスを作成
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    user_id = 'user_id_example' # str |  (オプション)
    badge_id = 'badge_id_example' # str |  (オプション)
    type = 3.4 # float |  (オプション)
    displayed_on_comments = True # bool |  (オプション)
    limit = 3.4 # float |  (オプション)
    skip = 3.4 # float |  (オプション)

    try:
        api_response = api_instance.get_user_badges(tenant_id, user_id=user_id, badge_id=badge_id, type=type, displayed_on_comments=displayed_on_comments, limit=limit, skip=skip)
        print("The response of DefaultApi->get_user_badges:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_user_badges: %s\n" % e)
[inline-code-end]