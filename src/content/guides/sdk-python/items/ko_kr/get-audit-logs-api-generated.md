## 매개변수

| 이름 | 유형 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| limit | number | query | No |  |
| skip | number | query | No |  |
| order | string | query | No |  |
| after | number | query | No |  |
| before | number | query | No |  |

## 응답

반환: [`GetAuditLogsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_audit_logs_response.py)

## 예시

[inline-code-attrs-start title = 'get_audit_logs 예시'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.default_api import GetAuditLogsOptions
from client.models.get_audit_logs_response import GetAuditLogsResponse
from client.models.sortdir import SORTDIR
from client.rest import ApiException
from pprint import pprint

# 호스트 정의는 선택 사항이며 기본값은 https://fastcomments.com 입니다
# 지원되는 모든 구성 매개변수 목록은 configuration.py를 참조하십시오.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# 클라이언트는 인증 및 권한 부여 매개변수를 설정해야 합니다
# API 서버 보안 정책에 따라.
# 아래에 각 인증 방법에 대한 예제가 제공되며, 귀하의 인증 사용 사례에 맞는 예제를 사용하십시오.
# 인증 사용 사례를 만족합니다.

# API 키 인증 구성: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 필요에 따라 API 키에 대한 접두사(예: Bearer)를 설정하려면 아래 주석을 해제하십시오
# configuration.api_key_prefix['api_key'] = 'Bearer'

# API 클라이언트 인스턴스로 컨텍스트에 들어갑니다
with client.ApiClient(configuration) as api_client:
    # API 클래스의 인스턴스를 생성합니다
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    limit = 3.4 # float |  (옵션)
    skip = 3.4 # float |  (옵션)
    order = client.SORTDIR() # SORTDIR |  (옵션)
    after = 3.4 # float |  (옵션)
    before = 3.4 # float |  (옵션)

    try:
        api_response = api_instance.get_audit_logs(tenant_id, GetAuditLogsOptions(limit=limit, skip=skip, order=order, after=after, before=before))
        print("The response of DefaultApi->get_audit_logs:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_audit_logs: %s\n" % e)
[inline-code-end]