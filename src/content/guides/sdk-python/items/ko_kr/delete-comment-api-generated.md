## 매개변수

| 이름 | 타입 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |
| contextUserId | string | query | No |  |
| isLive | boolean | query | No |  |

## 응답

반환: [`DeleteComment200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/delete_comment200_response.py)

## 예제

[inline-code-attrs-start title = 'delete_comment 예제'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.delete_comment200_response import DeleteComment200Response
from client.rest import ApiException
from pprint import pprint

# 호스트 정의는 선택 사항이며 기본값은 https://fastcomments.com 입니다
# 지원되는 모든 구성 매개변수 목록은 configuration.py 참조
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# 클라이언트는 인증 및 권한 부여 매개변수를
# API 서버 보안 정책에 따라 구성해야 합니다.
# 각 인증 방식에 대한 예제가 아래에 제공됩니다. 다음 예제들 중에서
# 귀하의 인증 사용 사례를 만족하는 예제를 사용하세요.
# API 키 인증 구성: api_key
# 필요 시 API 키에 대한 접두사 (e.g. Bearer)를 설정하려면 아래의 주석을 해제하세요

# API 클라이언트 인스턴스를 사용하여 컨텍스트에 진입합니다
with client.ApiClient(configuration) as api_client:
    # API 클래스의 인스턴스 생성
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    context_user_id = 'context_user_id_example' # str |  (선택 사항)
    is_live = True # bool |  (선택 사항)

    try:
        api_response = api_instance.delete_comment(tenant_id, id, context_user_id=context_user_id, is_live=is_live)
        print("The response of DefaultApi->delete_comment:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->delete_comment: %s\n" % e)
[inline-code-end]