## 매개변수

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 예 |  |
| id | string | path | 예 |  |

## 응답

반환: [`APIGetCommentResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_get_comment_response.py)

## 예제

[inline-code-attrs-start title = 'get_comment 예제'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.api_get_comment_response import APIGetCommentResponse
from client.rest import ApiException
from pprint import pprint

# 호스트 정의는 선택사항이며 기본값은 https://fastcomments.com 입니다
# 모든 지원되는 구성 매개변수 목록은 configuration.py 를 참조하세요.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# 클라이언트는 인증 및 인가 매개변수를 구성해야 합니다
# API 서버 보안 정책에 따라.
# 아래에 각 인증 방식의 예제가 제공됩니다. 자신의 인증 케이스에 맞는 예제를 사용하세요.

# API 키 인증 설정: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 필요하다면 아래 주석을 해제하여 API 키의 접두사(e.g. Bearer)를 설정하세요
# configuration.api_key_prefix['api_key'] = 'Bearer'

# API 클라이언트 인스턴스와 함께 컨텍스트에 진입합니다
with client.ApiClient(configuration) as api_client:
    # API 클래스의 인스턴스를 생성합니다
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 

    try:
        api_response = api_instance.get_comment(tenant_id, id)
        print("The response of DefaultApi->get_comment:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_comment: %s\n" % e)
[inline-code-end]