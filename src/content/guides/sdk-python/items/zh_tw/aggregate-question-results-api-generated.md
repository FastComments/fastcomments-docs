## 參數

| 名稱 | 類型 | 位置 | 必填 | 說明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| questionId | string | query | 否 |  |
| questionIds | array | query | 否 |  |
| urlId | string | query | 否 |  |
| timeBucket | string | query | 否 |  |
| startDate | string | query | 否 |  |
| forceRecalculate | boolean | query | 否 |  |

## 回應

回傳：[`AggregateQuestionResultsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/aggregate_question_results_response.py)

## 範例

[inline-code-attrs-start title = 'aggregate_question_results 範例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.aggregate_question_results_response import AggregateQuestionResultsResponse
from client.models.aggregate_time_bucket import AggregateTimeBucket
from client.rest import ApiException
from pprint import pprint

# 定義 host 是可選的，預設為 https://fastcomments.com
# 請參閱 configuration.py 以取得所有支援的設定參數列表。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# 用戶端必須依 API 伺服器的安全性政策設定驗證與授權參數。
# 下方提供每種驗證方法的範例，
# 請使用符合您驗證情境的範例。

# 設定 API key 授權：api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 如需要，取消註解下列設定以為 API key 設定前綴（例如 Bearer）
# configuration.api_key_prefix['api_key'] = 'Bearer'

# 以 API client 實例開啟一個上下文
with client.ApiClient(configuration) as api_client:
    # 建立 API 類別的實例
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    question_id = 'question_id_example' # str |  (選用)
    question_ids = ['question_ids_example'] # List[str] |  (選用)
    url_id = 'url_id_example' # str |  (選用)
    time_bucket = client.AggregateTimeBucket() # AggregateTimeBucket |  (選用)
    start_date = '2013-10-20T19:20:30+01:00' # datetime |  (選用)
    force_recalculate = True # bool |  (選用)

    try:
        api_response = api_instance.aggregate_question_results(tenant_id, question_id=question_id, question_ids=question_ids, url_id=url_id, time_bucket=time_bucket, start_date=start_date, force_recalculate=force_recalculate)
        print("The response of DefaultApi->aggregate_question_results:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->aggregate_question_results: %s\n" % e)
[inline-code-end]