---
## パラメータ

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| commentId | string | query | Yes |  |
| direction | string | query | Yes |  |
| userId | string | query | No |  |
| anonUserId | string | query | No |  |

## レスポンス

戻り値: [`VoteResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/vote_response.py)

## 例

[inline-code-attrs-start title = 'create_vote の例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.vote_response import VoteResponse
from client.rest import ApiException
from pprint import pprint

# ホストの定義はオプションで、デフォルトは https://fastcomments.com です
# See configuration.py for a list of all supported configuration parameters.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# クライアントは認証と認可のパラメータを
# API サーバーのセキュリティポリシーに従って設定する必要があります。
# 各認証方式の例を以下に示します。
# ご利用の認証ユースケースに合う例を使用してください。

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 必要に応じて API キー用のプレフィックス（例: Bearer）を設定するには以下の行のコメントアウトを解除してください
# configuration.api_key_prefix['api_key'] = 'Bearer'

# API クライアントのインスタンスを使ってコンテキストに入ります
with client.ApiClient(configuration) as api_client:
    # API クラスのインスタンスを作成します
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    comment_id = 'comment_id_example' # str | 
    direction = 'direction_example' # str | 
    user_id = 'user_id_example' # str |  (オプション)
    anon_user_id = 'anon_user_id_example' # str |  (オプション)

    try:
        api_response = api_instance.create_vote(tenant_id, comment_id, direction, user_id=user_id, anon_user_id=anon_user_id)
        print("The response of DefaultApi->create_vote:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->create_vote: %s\n" % e)
[inline-code-end]

---