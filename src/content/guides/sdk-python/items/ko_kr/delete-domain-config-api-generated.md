## 매개변수

| 이름 | 형식 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 예 |  |
| domain | string | path | 예 |  |

## 응답

반환: [`DeleteDomainConfigResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/delete_domain_config_response.py)

## 예제

[inline-code-attrs-start title = 'delete_domain_config 예제'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.delete_domain_config_response import DeleteDomainConfigResponse
from client.rest import ApiException
from pprint import pprint

# 호스트 설정은 선택 사항이며 기본값은 https://fastcomments.com 입니다
# 지원되는 모든 구성 매개변수 목록은 configuration.py를 참조하세요.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# 클라이언트는 인증 및 권한 부여 매개변수를
# API 서버 보안 정책에 따라 구성해야 합니다.
# 각 인증 방법에 대한 예제가 아래에 제공됩니다. 사용 사례에
# 맞는 예제를 사용하세요.
# API 키 인증 구성: api_key
# 필요한 경우 API 키에 대한 접두사(예: Bearer)를 설정하려면 아래의 주석을 해제하세요

# API 클라이언트 인스턴스로 컨텍스트에 들어갑니다
with client.ApiClient(configuration) as api_client:
    # API 클래스의 인스턴스를 생성합니다
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    domain = 'domain_example' # str | 

    try:
        api_response = api_instance.delete_domain_config(tenant_id, domain)
        print("The response of DefaultApi->delete_domain_config:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->delete_domain_config: %s\n" % e)
[inline-code-end]