## 參數

| 名稱 | 型別 | 位置 | 必填 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| questionId | string | query | No |  |
| questionIds | array | query | No |  |
| urlId | string | query | No |  |
| timeBucket | string | query | No |  |
| startDate | string | query | No |  |
| forceRecalculate | boolean | query | No |  |

## 回應

返回：[`AggregateQuestionResultsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/aggregate_question_results_response.py)

## 範例

[inline-code-attrs-start title = 'aggregate_question_results 範例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.default_api import AggregateQuestionResultsOptions
from client.models.aggregate_question_results_response import AggregateQuestionResultsResponse
from client.models.aggregate_time_bucket import AggregateTimeBucket
from client.rest import ApiException
from pprint import pprint

# 定義主機是可選的，預設為 https://fastcomments.com
# 請參閱 configuration.py 以取得所有支援的設定參數列表。
# 客戶端必須根據 API 伺服器的安全政策配置驗證與授權參數
# 依照 API 伺服器的安全政策。
# 以下提供每種驗證方法的範例，使用符合您驗證需求的範例。
# 滿足您的驗證使用情境。
# 配置 API 金鑰授權：api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 如有需要，取消註解以下程式碼以為 API 金鑰設定前綴（例如 Bearer）# configuration.api_key_prefix['api_key'] = 'Bearer'

# 以 API 客戶端實例進入上下文
with client.ApiClient(configuration) as api_client:
    # 建立 API 類別的實例
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    question_id = 'question_id_example' # str |  （可選）
    question_ids = ['question_ids_example'] # List[str] |  （可選）
    url_id = 'url_id_example' # str |  （可選）
    time_bucket = client.AggregateTimeBucket() # AggregateTimeBucket |  （可選）
    start_date = '2013-10-20T19:20:30+01:00' # datetime |  （可選）
    force_recalculate = True # bool |  （可選）

    try:
        api_response = api_instance.aggregate_question_results(tenant_id, AggregateQuestionResultsOptions(question_id=question_id, question_ids=question_ids, url_id=url_id, time_bucket=time_bucket, start_date=start_date, force_recalculate=force_recalculate))
        print("The response of DefaultApi->aggregate_question_results:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->aggregate_question_results: %s\n" % e)
[inline-code-end]