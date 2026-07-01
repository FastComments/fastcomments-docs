## 매개변수

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| questionId | string | query | No |  |
| questionIds | array | query | No |  |
| urlId | string | query | No |  |
| timeBucket | string | query | No |  |
| startDate | string | query | No |  |
| forceRecalculate | boolean | query | No |  |

## 응답

반환: [`AggregateQuestionResultsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/aggregate_question_results_response.py)

## 예제

[inline-code-attrs-start title = 'aggregate_question_results 예제'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.default_api import AggregateQuestionResultsOptions
from client.models.aggregate_question_results_response import AggregateQuestionResultsResponse
from client.models.aggregate_time_bucket import AggregateTimeBucket
from client.rest import ApiException
from pprint import pprint

# 호스트 정의는 선택 사항이며 기본값은 https://fastcomments.com 입니다
# configuration.py에서 지원되는 모든 구성 매개변수 목록을 확인하십시오.
# 클라이언트는 API 서버 보안 정책에 따라 인증 및 권한 부여 매개변수를 구성해야 합니다.
# 아래에 각 인증 방법에 대한 예제가 제공됩니다. 사용 사례에 맞는 예제를 사용하십시오.

# API 키 인증 구성: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 필요에 따라 API 키에 접두사(예: Bearer)를 설정하려면 아래 주석을 해제하십시오.
# configuration.api_key_prefix['api_key'] = 'Bearer'

# API 클라이언트 인스턴스와 함께 컨텍스트에 들어갑니다
with client.ApiClient(configuration) as api_client:
    # API 클래스의 인스턴스를 생성합니다
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    question_id = 'question_id_example' # str |  (옵션)
    question_ids = ['question_ids_example'] # List[str] |  (옵션)
    url_id = 'url_id_example' # str |  (옵션)
    time_bucket = client.AggregateTimeBucket() # AggregateTimeBucket |  (옵션)
    start_date = '2013-10-20T19:20:30+01:00' # datetime |  (옵션)
    force_recalculate = True # bool |  (옵션)

    try:
        api_response = api_instance.aggregate_question_results(tenant_id, AggregateQuestionResultsOptions(question_id=question_id, question_ids=question_ids, url_id=url_id, time_bucket=time_bucket, start_date=start_date, force_recalculate=force_recalculate))
        print("The response of DefaultApi->aggregate_question_results:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->aggregate_question_results: %s\n" % e)
[inline-code-end]