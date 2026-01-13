## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| urlId | string | query | いいえ |  |
| userId | string | query | いいえ |  |
| startDate | string | query | いいえ |  |
| questionId | string | query | いいえ |  |
| questionIds | string | query | いいえ |  |
| skip | number | query | いいえ |  |

## レスポンス

戻り値: [`GetQuestionResults200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_question_results200_response.py)

## 例

[inline-code-attrs-start title = 'get_question_results の例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_question_results200_response import GetQuestionResults200Response
from client.rest import ApiException
from pprint import pprint

# ホストの定義はオプションで、デフォルトは https://fastcomments.com です
# サポートされているすべての設定パラメータの一覧は configuration.py を参照してください。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# クライアントは API サーバーのセキュリティポリシーに従って認証および認可パラメータを設定する必要があります。
# 各認証方式の例は下に示されています。自身の認証ユースケースに合う例を使用してください。

# API キー認証を設定: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 必要に応じて、API キーのプレフィックス（例: Bearer）を設定するには下のコメントを外してください
# configuration.api_key_prefix['api_key'] = 'Bearer'

# API クライアントのインスタンスを使ってコンテキストを開きます
with client.ApiClient(configuration) as api_client:
    # API クラスのインスタンスを作成
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str |  （オプション）
    user_id = 'user_id_example' # str |  （オプション）
    start_date = 'start_date_example' # str |  （オプション）
    question_id = 'question_id_example' # str |  （オプション）
    question_ids = 'question_ids_example' # str |  （オプション）
    skip = 3.4 # float |  （オプション）

    try:
        api_response = api_instance.get_question_results(tenant_id, url_id=url_id, user_id=user_id, start_date=start_date, question_id=question_id, question_ids=question_ids, skip=skip)
        print("The response of DefaultApi->get_question_results:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_question_results: %s\n" % e)
[inline-code-end]