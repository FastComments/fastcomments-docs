## パラメータ

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| userId | string | path | はい |  |

## レスポンス

戻り値: [`APIGetUserBadgeProgressResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_get_user_badge_progress_response.py)

## 例

[inline-code-attrs-start title = 'get_user_badge_progress_by_user_id の例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.api_get_user_badge_progress_response import APIGetUserBadgeProgressResponse
from client.rest import ApiException
from pprint import pprint

# ホストの定義は任意で、デフォルトは https://fastcomments.com です
# サポートされているすべての設定パラメータについては configuration.py を参照してください。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# クライアントは認証および認可のパラメータを構成する必要があります
# API サーバーのセキュリティポリシーに従ってください。
# 以下に各認証方法の例を示します。ご自身の認証ユースケースに合う例を使用してください。
# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 必要な場合は下の行のコメントアウトを外して API キーのプレフィックス（例: Bearer）を設定してください
# configuration.api_key_prefix['api_key'] = 'Bearer'

# API クライアントのインスタンスを伴うコンテキストを開きます
with client.ApiClient(configuration) as api_client:
    # API クラスのインスタンスを作成します
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    user_id = 'user_id_example' # str | 

    try:
        api_response = api_instance.get_user_badge_progress_by_user_id(tenant_id, user_id)
        print("The response of DefaultApi->get_user_badge_progress_by_user_id:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_user_badge_progress_by_user_id: %s\n" % e)
[inline-code-end]