현재 페이지에 접속 중인 뷰어: 현재 웹소켓 세션이 해당 페이지에 구독되어 있는 사람들.
Returns anonCount + totalCount (room-wide subscribers, including anon viewers we don't enumerate).

## 매개변수

| 이름 | 타입 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | 페이지 URL 식별자 (서버 측에서 정리됨). |
| afterName | string | query | No | 커서: 이전 응답의 nextAfterName을 전달하세요. |
| afterUserId | string | query | No | 커서 동점 해제: 이전 응답의 nextAfterUserId를 전달하세요. afterName이 설정된 경우 이름이 같아 항목이 누락되지 않도록 필요합니다. |

## 응답

반환: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/page_users_online_response.py)

## 예제

[inline-code-attrs-start title = 'get_online_users 예제'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.page_users_online_response import PageUsersOnlineResponse
from client.rest import ApiException
from pprint import pprint

# 호스트를 정의하는 것은 선택 사항이며 기본값은 https://fastcomments.com입니다
# 모든 지원되는 구성 매개변수 목록은 configuration.py를 참조하세요.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# API 클라이언트 인스턴스와 함께 컨텍스트로 들어갑니다
with client.ApiClient(configuration) as api_client:
    # API 클래스의 인스턴스를 생성합니다
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | 페이지 URL 식별자 (서버 측에서 정리됨).
    after_name = 'after_name_example' # str | 커서: 이전 응답의 nextAfterName을 전달하세요. (선택 사항)
    after_user_id = 'after_user_id_example' # str | 커서 동점 해제: 이전 응답의 nextAfterUserId를 전달하세요. afterName이 설정된 경우 이름이 같아 항목이 누락되지 않도록 필수입니다. (선택 사항)

    try:
        api_response = api_instance.get_online_users(tenant_id, url_id, after_name=after_name, after_user_id=after_user_id)
        print("The response of PublicApi->get_online_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_online_users: %s\n" % e)
[inline-code-end]