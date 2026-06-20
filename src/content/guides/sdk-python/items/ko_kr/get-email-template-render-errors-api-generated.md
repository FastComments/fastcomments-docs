## 매개변수

| 이름 | 유형 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 예 |  |
| id | string | path | 예 |  |
| skip | number | query | 아니오 |  |

## 응답

반환: [`GetEmailTemplateRenderErrorsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_email_template_render_errors_response.py)

## 예제

[inline-code-attrs-start title = 'get_email_template_render_errors 예제'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_email_template_render_errors_response import GetEmailTemplateRenderErrorsResponse
from client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to https://fastcomments.com
# 지원되는 모든 구성 매개변수 목록은 configuration.py를 참조하세요.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# 클라이언트는 API 서버 보안 정책에 따라 인증 및 권한 부여 매개변수를 구성해야 합니다.
# 각 인증 방식에 대한 예제가 아래에 제공되며, 사용 사례에 맞는 예제를 사용하세요.

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
# 필요하면 API 키 접두사(예: Bearer)를 설정하려면 아래의 주석을 해제하세요
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Enter a context with an instance of the API client
# API 클라이언트 인스턴스로 컨텍스트에 진입합니다
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    # API 클래스의 인스턴스를 생성합니다
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    skip = 3.4 # float |  (선택 사항)

    try:
        api_response = api_instance.get_email_template_render_errors(tenant_id, id, skip=skip)
        print("The response of DefaultApi->get_email_template_render_errors:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_email_template_render_errors: %s\n" % e)
[inline-code-end]