---
## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| questionId | string | query | いいえ |  |
| questionIds | array | query | いいえ |  |
| urlId | string | query | いいえ |  |
| timeBucket | string | query | いいえ |  |
| startDate | string | query | いいえ |  |
| forceRecalculate | boolean | query | いいえ |  |

## レスポンス

戻り値: [`AggregateQuestionResultsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/aggregate_question_results_response.py)

## 例

[inline-code-attrs-start title = 'aggregate_question_results の例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.aggregate_question_results_response import AggregateQuestionResultsResponse
from client.models.aggregate_time_bucket import AggregateTimeBucket
from client.rest import ApiException
from pprint import pprint

# ホストの定義は任意で、デフォルトは https://fastcomments.com です
# サポートされているすべての設定パラメータの一覧は configuration.py を参照してください。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# クライアントは認証および認可のパラメータを設定する必要があります
# APIサーバーのセキュリティポリシーに従ってください。
# 以下に各認証方法の例を示します。あなたの認証ユースケースに合う例を使用してください。

# APIキー認証の設定: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 必要に応じて以下のコメントを外して APIキーのプレフィックス（例: Bearer）を設定してください
# configuration.api_key_prefix['api_key'] = 'Bearer'

# APIクライアントのインスタンスでコンテキストを開始します
with client.ApiClient(configuration) as api_client:
    # APIクラスのインスタンスを作成します
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    question_id = 'question_id_example' # str |  （省略可）
    question_ids = ['question_ids_example'] # List[str] |  （省略可）
    url_id = 'url_id_example' # str |  （省略可）
    time_bucket = client.AggregateTimeBucket() # AggregateTimeBucket |  （省略可）
    start_date = '2013-10-20T19:20:30+01:00' # datetime |  （省略可）
    force_recalculate = True # bool |  （省略可）

    try:
        api_response = api_instance.aggregate_question_results(tenant_id, question_id=question_id, question_ids=question_ids, url_id=url_id, time_bucket=time_bucket, start_date=start_date, force_recalculate=force_recalculate)
        print("The response of DefaultApi->aggregate_question_results:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->aggregate_question_results: %s\n" % e)
[inline-code-end]

---