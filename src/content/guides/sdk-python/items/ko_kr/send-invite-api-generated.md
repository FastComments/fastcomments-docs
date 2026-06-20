## 매개변수

| 이름 | 형식 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 예 |  |
| id | string | path | 예 |  |
| fromName | string | query | 예 |  |

## 응답

반환: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_empty_response.py)

## 예제

[inline-code-attrs-start title = 'send_invite 예제'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.api_empty_response import APIEmptyResponse
from client.rest import ApiException
from pprint import pprint

# 호스트를 정의하는 것은 선택 사항이며 기본값은 https://fastcomments.com 입니다
# configuration.py에서 지원되는 모든 구성 매개변수 목록을 확인하세요.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# 클라이언트는 인증 및 권한 부여 매개변수를
# API 서버 보안 정책에 따라 구성해야 합니다.
# 아래에는 각 인증 방식에 대한 예제가 제공됩니다. 사용 사례에
# 맞는 예제를 사용하세요.

# API 키 인증 구성: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# 필요하면 아래 주석을 해제하여 API 키 접두사(예: Bearer)를 설정하세요
# configuration.api_key_prefix['api_key'] = 'Bearer'

# API 클라이언트 인스턴스로 컨텍스트에 진입합니다
with client.ApiClient(configuration) as api_client:
    # API 클래스의 인스턴스를 생성합니다
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    from_name = 'from_name_example' # str | 

    try:
        api_response = api_instance.send_invite(tenant_id, id, from_name)
        print("The response of DefaultApi->send_invite:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->send_invite: %s\n" % e)
[inline-code-end]