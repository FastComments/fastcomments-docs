## 매개변수

| 이름 | 타입 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 예 |  |
| id | string | path | 예 |  |
| deleteComments | boolean | query | 아니요 |  |
| commentDeleteMode | string | query | 아니요 |  |

## 응답

반환: [`DeleteSSOUserAPIResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/delete_sso_user_api_response.py)

## 예제

[inline-code-attrs-start title = 'delete_sso_user 예제'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.delete_sso_user_api_response import DeleteSSOUserAPIResponse
from client.rest import ApiException
from pprint import pprint

# 호스트 정의는 선택 사항이며 기본값은 https://fastcomments.com
# 모든 지원되는 구성 매개변수 목록은 configuration.py를 참조하세요.
# 클라이언트는 인증 및 권한 부여 매개변수를
# API 서버 보안 정책에 따라 구성해야 합니다.
# 각 인증 방법에 대한 예제가 아래에 제공됩니다. 아래 예제 중
# 귀하의 인증 사용 사례에 맞는 것을 사용하세요.
# API 키 인증 구성: api_key
# 필요하면 API 키에 접두사(e.g. Bearer)를 설정하려면 아래 주석을 해제하세요
# API 클라이언트 인스턴스와 함께 컨텍스트에 진입합니다
with client.ApiClient(configuration) as api_client:
    # API 클래스의 인스턴스를 생성합니다
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    delete_comments = True # bool |  (선택 사항)
    comment_delete_mode = 'comment_delete_mode_example' # str |  (선택 사항)

    try:
        api_response = api_instance.delete_sso_user(tenant_id, id, delete_comments=delete_comments, comment_delete_mode=comment_delete_mode)
        print("The response of DefaultApi->delete_sso_user:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->delete_sso_user: %s\n" % e)
[inline-code-end]