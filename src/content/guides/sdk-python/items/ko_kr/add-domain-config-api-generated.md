## 매개변수

| 이름 | 유형 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 예 |  |

## 응답

반환: [`AddDomainConfig200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/add_domain_config200_response.py)

## 예제

[inline-code-attrs-start title = 'add_domain_config 예제'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.add_domain_config200_response import AddDomainConfig200Response
from client.models.add_domain_config_params import AddDomainConfigParams
from client.rest import ApiException
from pprint import pprint

# 호스트 설정은 선택 사항이며 기본값은 https://fastcomments.com 입니다
# See configuration.py for a list of all supported configuration parameters.
# 클라이언트는 인증 및 권한 부여 매개변수를
# API 서버 보안 정책에 따라 구성해야 합니다.
# 각 인증 방법에 대한 예제가 아래에 제공됩니다, 사용 사례에
# 맞는 예제를 사용하세요.
# Configure API key authorization: api_key
# 필요한 경우 API 키에 대한 접두사(예: Bearer)를 설정하려면 아래 주석을 해제하세요
# API 클라이언트 인스턴스로 컨텍스트에 진입합니다
    # Create an instance of the API class
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    add_domain_config_params = client.AddDomainConfigParams() # AddDomainConfigParams | 

    try:
        api_response = api_instance.add_domain_config(tenant_id, add_domain_config_params)
        print("The response of DefaultApi->add_domain_config:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->add_domain_config: %s\n" % e)
[inline-code-end]