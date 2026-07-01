## パラメータ

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| questionId | string | query | いいえ |  |
| questionIds | array | query | いいえ |  |
| urlId | string | query | いいえ |  |
| timeBucket | string | query | いいえ |  |
| startDate | string | query | いいえ |  |
| forceRecalculate | boolean | query | いいえ |  |

## レスポンス

返り値: [`AggregateQuestionResultsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/aggregate_question_results_response.py)

## 例

[inline-code-attrs-start title = 'aggregate_question_results の例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.default_api import AggregateQuestionResultsOptions
from client.models.aggregate_question_results_response import AggregateQuestionResultsResponse
from client.models.aggregate_time_bucket import AggregateTimeBucket
from client.rest import ApiException
from pprint import pprint

# ホストの定義はオプションで、デフォルトは https://fastcomments.com です
# サポートされているすべての設定パラメータの一覧は configuration.py を参照してください。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# クライアントは認証および認可パラメータを設定しなければなりません
# API サーバーのセキュリティポリシーに従って。
# 各認証方法の例が以下に提供されています。使用する例は
# あなたの認証ユースケースに合致するものです。

# API キー認証を設定します: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 必要に応じて、API キーのプレフィックス（例: Bearer）を設定するには以下のコメントを解除してください
# configuration.api_key_prefix['api_key'] = 'Bearer'

# API クライアントのインスタンスでコンテキストに入ります
with client.ApiClient(configuration) as api_client:
    # API クラスのインスタンスを作成します
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    question_id = 'question_id_example' # str |  （オプション）
    question_ids = ['question_ids_example'] # List[str] |  （オプション）
    url_id = 'url_id_example' # str |  （オプション）
    time_bucket = client.AggregateTimeBucket() # AggregateTimeBucket |  （オプション）
    start_date = '2013-10-20T19:20:30+01:00' # datetime |  （オプション）
    force_recalculate = True # bool |  （オプション）

    try:
        api_response = api_instance.aggregate_question_results(tenant_id, AggregateQuestionResultsOptions(question_id=question_id, question_ids=question_ids, url_id=url_id, time_bucket=time_bucket, start_date=start_date, force_recalculate=force_recalculate))
        print("The response of DefaultApi->aggregate_question_results:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->aggregate_question_results: %s\n" % e)
[inline-code-end]