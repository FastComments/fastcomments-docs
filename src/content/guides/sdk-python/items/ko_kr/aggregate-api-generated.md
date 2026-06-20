---
문서를 그룹화(groupBy가 제공되는 경우)하고 여러 연산을 적용하여 집계합니다. 다양한 연산(예: sum, countDistinct, avg 등)을 지원합니다.

## 매개변수

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 예 |  |
| parentTenantId | string | query | 아니요 |  |
| includeStats | boolean | query | 아니요 |  |

## 응답

반환: [`AggregateResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/aggregate_response.py)

## 예제

[inline-code-attrs-start title = 'aggregate 예제'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.aggregate_response import AggregateResponse
from client.models.aggregation_request import AggregationRequest
from client.rest import ApiException
from pprint import pprint

# 호스트 정의는 선택사항이며 기본값은 https://fastcomments.com 입니다
# 지원되는 모든 구성 매개변수 목록은 configuration.py를 참조하세요.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# 클라이언트는 인증 및 권한 부여 매개변수를 구성해야 합니다
# API 서버 보안 정책에 따라.
# 각 인증 방법에 대한 예제가 아래에 제공됩니다. 다음 중 귀하의 인증 사용 사례에
# 부합하는 예제를 사용하세요.

# API 키 인증 구성: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 필요하면 API 키에 대한 접두사(예: Bearer)를 설정하려면 아래의 주석을 해제하세요
# configuration.api_key_prefix['api_key'] = 'Bearer'

# API 클라이언트 인스턴스와 함께 컨텍스트에 진입하세요
with client.ApiClient(configuration) as api_client:
    # API 클래스의 인스턴스를 생성하세요
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    aggregation_request = client.AggregationRequest() # AggregationRequest | 
    parent_tenant_id = 'parent_tenant_id_example' # str |  (선택 사항)
    include_stats = True # bool |  (선택 사항)

    try:
        api_response = api_instance.aggregate(tenant_id, aggregation_request, parent_tenant_id=parent_tenant_id, include_stats=include_stats)
        print("The response of DefaultApi->aggregate:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->aggregate: %s\n" % e)
[inline-code-end]

---