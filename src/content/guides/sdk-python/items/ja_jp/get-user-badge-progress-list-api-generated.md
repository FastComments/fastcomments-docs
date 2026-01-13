## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | クエリ | はい |  |
| userId | string | クエリ | いいえ |  |
| limit | number | クエリ | いいえ |  |
| skip | number | クエリ | いいえ |  |

## レスポンス

戻り値: [`GetUserBadgeProgressList200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_user_badge_progress_list200_response.py)

## 例

[inline-code-attrs-start title = 'get_user_badge_progress_list の例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_user_badge_progress_list200_response import GetUserBadgeProgressList200Response
from client.rest import ApiException
from pprint import pprint

# ホストの定義はオプションで、デフォルトは https://fastcomments.com です
# サポートされているすべての設定パラメータの一覧は configuration.py を参照してください。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# クライアントは認証および認可パラメータを
# API サーバーのセキュリティポリシーに従って設定する必要があります。
# 以下に各認証方式の例を示します。該当するものを使用してください。

# API キー認証の設定: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 必要に応じて API キーのプレフィックス（例: Bearer）を設定するには以下の行のコメントを外してください
# configuration.api_key_prefix['api_key'] = 'Bearer'

# API クライアントのインスタンスを使ってコンテキストに入ります
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    user_id = 'user_id_example' # str |  （オプション）
    limit = 3.4 # float |  （オプション）
    skip = 3.4 # float |  （オプション）

    try:
        api_response = api_instance.get_user_badge_progress_list(tenant_id, user_id=user_id, limit=limit, skip=skip)
        print("The response of DefaultApi->get_user_badge_progress_list:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_user_badge_progress_list: %s\n" % e)
[inline-code-end]

---