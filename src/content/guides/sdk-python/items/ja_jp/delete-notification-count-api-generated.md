## パラメーター

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |

## レスポンス

返却: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_empty_response.py)

## 例

[inline-code-attrs-start title = 'delete_notification_count の例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.api_empty_response import APIEmptyResponse
from client.rest import ApiException
from pprint import pprint

# ホストの定義はオプションで、デフォルトは https://fastcomments.com です
# サポートされている全ての設定パラメータの一覧は configuration.py を参照してください。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# クライアントは認証と認可のパラメータを設定する必要があります
# API サーバーのセキュリティポリシーに従ってください。
# 各認証方式の例は以下に示しています。
# 自身の認証ユースケースに合う例を使用してください。

# APIキー認証を設定: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 必要に応じて API キーのプレフィックス（例: Bearer）を設定するには下の行のコメントを外してください
# configuration.api_key_prefix['api_key'] = 'Bearer'

# API クライアントのインスタンスでコンテキストに入ります
with client.ApiClient(configuration) as api_client:
    # API クラスのインスタンスを作成します
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 

    try:
        api_response = api_instance.delete_notification_count(tenant_id, id)
        print("The response of DefaultApi->delete_notification_count:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->delete_notification_count: %s\n" % e)
[inline-code-end]