## 매개변수

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 예 |  |
| commentId | string | query | 아니오 |  |
| externalId | string | query | 아니오 |  |
| eventType | string | query | 아니오 |  |
| type | string | query | 아니오 |  |
| domain | string | query | 아니오 |  |
| attemptCountGT | number | query | 아니오 |  |
| skip | number | query | 아니오 |  |

## 응답

반환: [`GetPendingWebhookEventsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_pending_webhook_events_response.py)

## 예제

[inline-code-attrs-start title = 'get_pending_webhook_events 예제'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_pending_webhook_events_response import GetPendingWebhookEventsResponse
from client.rest import ApiException
from pprint import pprint

# 호스트 정의는 선택 사항이며 기본값은 https://fastcomments.com 입니다
# 지원되는 모든 구성 매개변수 목록은 configuration.py 를 참조하세요.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# 클라이언트는 API 서버 보안 정책에 따라 인증 및 권한 부여 매개변수를 구성해야 합니다.
# 아래에 각 인증 방법에 대한 예제가 제공됩니다. 사용 사례에 맞는 예제를 사용하세요.

# API 키 인증 구성: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 필요에 따라 API 키에 대한 접두사(예: Bearer)를 설정하려면 아래의 주석을 해제하세요
# configuration.api_key_prefix['api_key'] = 'Bearer'

# API 클라이언트 인스턴스와 함께 컨텍스트에 진입합니다
with client.ApiClient(configuration) as api_client:
    # API 클래스의 인스턴스를 생성합니다
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    comment_id = 'comment_id_example' # str |  (선택 사항)
    external_id = 'external_id_example' # str |  (선택 사항)
    event_type = 'event_type_example' # str |  (선택 사항)
    type = 'type_example' # str |  (선택 사항)
    domain = 'domain_example' # str |  (선택 사항)
    attempt_count_gt = 3.4 # float |  (선택 사항)
    skip = 3.4 # float |  (선택 사항)

    try:
        api_response = api_instance.get_pending_webhook_events(tenant_id, comment_id=comment_id, external_id=external_id, event_type=event_type, type=type, domain=domain, attempt_count_gt=attempt_count_gt, skip=skip)
        print("The response of DefaultApi->get_pending_webhook_events:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_pending_webhook_events: %s\n" % e)
[inline-code-end]