## 參數

| 名稱 | 類型 | 位置 | 必填 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| questionId | string | query | No |  |
| questionIds | array | query | No |  |
| urlId | string | query | No |  |
| startDate | string | query | No |  |
| forceRecalculate | boolean | query | No |  |
| minValue | number | query | No |  |
| maxValue | number | query | No |  |
| limit | number | query | No |  |

## 回應

回傳: [`CombineQuestionResultsWithCommentsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/combine_question_results_with_comments_response.py)

## 範例

[inline-code-attrs-start title = 'combine_comments_with_question_results 範例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.combine_question_results_with_comments_response import CombineQuestionResultsWithCommentsResponse
from client.rest import ApiException
from pprint import pprint

# 指定 host 為選用，預設為 https://fastcomments.com
# 請參閱 configuration.py 以取得所有支援的組態參數清單。
# 用戶端必須設定驗證與授權參數
# 以符合 API 伺服器的安全性政策。
# 下方提供每種驗證方法的範例，
# 請使用符合您驗證情境的範例。

# Configure API key authorization: api_key
# 如有需要，取消註解下方以設定 API 金鑰的前綴 (例如 Bearer)
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Enter a context with an instance of the API client
# Create an instance of the API class
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    question_id = 'question_id_example' # str |  (可選)
    question_ids = ['question_ids_example'] # List[str] |  (可選)
    url_id = 'url_id_example' # str |  (可選)
    start_date = '2013-10-20T19:20:30+01:00' # datetime |  (可選)
    force_recalculate = True # bool |  (可選)
    min_value = 3.4 # float |  (可選)
    max_value = 3.4 # float |  (可選)
    limit = 3.4 # float |  (可選)

    try:
        api_response = api_instance.combine_comments_with_question_results(tenant_id, question_id=question_id, question_ids=question_ids, url_id=url_id, start_date=start_date, force_recalculate=force_recalculate, min_value=min_value, max_value=max_value, limit=limit)
        print("The response of DefaultApi->combine_comments_with_question_results:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->combine_comments_with_question_results: %s\n" % e)
[inline-code-end]