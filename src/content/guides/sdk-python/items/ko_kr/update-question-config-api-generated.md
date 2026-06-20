---
## 매개변수

| 이름 | 유형 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 예 |  |
| id | string | path | 예 |  |

## 응답

반환: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_empty_response.py)

## 예제

[inline-code-attrs-start title = 'update_question_config 예제'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.api_empty_response import APIEmptyResponse
from client.models.update_question_config_body import UpdateQuestionConfigBody
from client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to https://fastcomments.com
# 호스트 정의는 선택 사항이며 기본값은 https://fastcomments.com 입니다
# See configuration.py for a list of all supported configuration parameters.
# 지원되는 모든 구성 매개변수 목록은 configuration.py를 참조하세요.

# The client must configure the authentication and authorization parameters
# 클라이언트는 인증 및 인가 매개변수를 구성해야 합니다
# in accordance with the API server security policy.
# API 서버의 보안 정책에 따라.
# Examples for each auth method are provided below, use the example that
# 각 인증 방법에 대한 예시는 아래에 제공됩니다.
# satisfies your auth use case.
# 귀하의 인증 사용 사례에 맞는 예제를 사용하세요.

# Configure API key authorization: api_key
# API 키 인증 구성: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
# 필요하다면 API 키에 대한 접두사(예: Bearer)를 설정하려면 아래의 주석을 해제하세요
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Enter a context with an instance of the API client
# API 클라이언트 인스턴스로 컨텍스트에 진입합니다
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    # API 클래스의 인스턴스를 생성합니다
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    update_question_config_body = client.UpdateQuestionConfigBody() # UpdateQuestionConfigBody | 

    try:
        api_response = api_instance.update_question_config(tenant_id, id, update_question_config_body)
        print("The response of DefaultApi->update_question_config:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->update_question_config: %s\n" % e)
[inline-code-end]

---