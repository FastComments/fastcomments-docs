## パラメータ

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| userId | string | query | いいえ |  |
| limit | number | query | いいえ |  |
| skip | number | query | いいえ |  |

## レスポンス

戻り値: [`APIGetUserBadgeProgressListResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_get_user_badge_progress_list_response.py)

## 例

[inline-code-attrs-start title = 'get_user_badge_progress_list の例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.api_get_user_badge_progress_list_response import APIGetUserBadgeProgressListResponse
from client.rest import ApiException
from pprint import pprint

# ホストの定義は任意で、デフォルトは https://fastcomments.com です
# サポートされているすべての設定パラメータの一覧は configuration.py を参照してください。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# クライアントは認証と認可のパラメータを設定する必要があります
# API サーバーのセキュリティポリシーに従ってください。
# 各認証方法の例は以下に示されています。 
# 自分のユースケースに合う例を使用してください。

# API キーによる認証を設定: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 必要に応じて、API キーのプレフィックス（例: Bearer）を設定するには下の行のコメントを外してください
# configuration.api_key_prefix['api_key'] = 'Bearer'

# API クライアントのインスタンスを用いてコンテキストに入ります
with client.ApiClient(configuration) as api_client:
    # API クラスのインスタンスを作成
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str |  
    user_id = 'user_id_example' # str |  （任意）
    limit = 3.4 # float |  （任意）
    skip = 3.4 # float |  （任意）

    try:
        api_response = api_instance.get_user_badge_progress_list(tenant_id, user_id=user_id, limit=limit, skip=skip)
        print("The response of DefaultApi->get_user_badge_progress_list:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_user_badge_progress_list: %s\n" % e)
[inline-code-end]