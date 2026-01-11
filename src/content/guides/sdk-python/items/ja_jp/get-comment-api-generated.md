## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | クエリ | はい |  |
| id | string | パス | はい |  |

## レスポンス

戻り値: [`GetComment200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_comment200_response.py)

## 例

[inline-code-attrs-start title = 'get_comment の例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_comment200_response import GetComment200Response
from client.rest import ApiException
from pprint import pprint

# ホストの定義は省略可能で、デフォルトは https://fastcomments.com です
# サポートされているすべての設定パラメータの一覧は configuration.py を参照してください。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# クライアントは認証および認可のパラメータを設定する必要があります。
# APIサーバーのセキュリティポリシーに従ってください。
# 下記に各認証方式の例を示します。該当する例を使用してください。
# ご自身のユースケースに合う例を利用してください。

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 必要に応じて下の行のコメントを外し、APIキーの接頭辞（例: Bearer）を設定してください
# configuration.api_key_prefix['api_key'] = 'Bearer'

# APIクライアントのインスタンスを用いてコンテキストに入る
with client.ApiClient(configuration) as api_client:
    # API クラスのインスタンスを作成
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 

    try:
        api_response = api_instance.get_comment(tenant_id, id)
        print("The response of DefaultApi->get_comment:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_comment: %s\n" % e)
[inline-code-end]