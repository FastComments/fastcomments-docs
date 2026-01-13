## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| id | string | path | はい |  |

## レスポンス

返り値: [`GetQuestionConfig200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_question_config200_response.py)

## 例

[inline-code-attrs-start title = 'get_question_config の例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_question_config200_response import GetQuestionConfig200Response
from client.rest import ApiException
from pprint import pprint

# ホストの定義はオプションで、デフォルトは https://fastcomments.com です
# サポートされているすべての設定パラメータの一覧は configuration.py を参照してください。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# クライアントは API サーバーのセキュリティポリシーに従って認証および認可パラメータを設定する必要があります。
# 各認証方法の例を以下に示します。自分の認証ケースに合った例を使用してください。

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 必要に応じて API キーのプレフィックス（例: Bearer）を設定するには下の行のコメントアウトを解除してください
# configuration.api_key_prefix['api_key'] = 'Bearer'

# API クライアントのインスタンスでコンテキストに入ります
with client.ApiClient(configuration) as api_client:
    # API クラスのインスタンスを作成します
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 

    try:
        api_response = api_instance.get_question_config(tenant_id, id)
        print("The response of DefaultApi->get_question_config:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_question_config: %s\n" % e)
[inline-code-end]

---