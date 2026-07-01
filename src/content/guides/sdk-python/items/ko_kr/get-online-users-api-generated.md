Currently-online viewers of a page: people whose websocket session is subscribed to the page right now.
Returns anonCount + totalCount (room-wide subscribers, including anon viewers we don't enumerate).

## 매개변수

| 이름 | 유형 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | 페이지 URL 식별자(서버 측에서 정리됨). |
| afterName | string | query | No | 커서: 이전 응답에서 nextAfterName을 전달합니다. |
| afterUserId | string | query | No | 커서 동점 해결: 이전 응답에서 nextAfterUserId를 전달합니다. afterName이 설정된 경우 이름 동점이 항목을 누락하지 않도록 필요합니다. |

## 응답

반환: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/page_users_online_response.py)

## 예시

[inline-code-attrs-start title = 'get_online_users 예제'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import GetOnlineUsersOptions
from client.models.page_users_online_response import PageUsersOnlineResponse
from client.rest import ApiException
from pprint import pprint

# 호스트 정의는 선택 사항이며 기본값은 https://fastcomments.com 입니다
# 지원되는 모든 구성 매개변수 목록은 configuration.py를 참조하십시오.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Enter a context with an instance of the API client
with client.ApiClient(configuration) as api_client:
    # API 클래스의 인스턴스를 생성합니다
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | 페이지 URL 식별자(서버 측에서 정리됨).
    after_name = 'after_name_example' # str | 커서: 이전 응답에서 nextAfterName을 전달합니다. (옵션)
    after_user_id = 'after_user_id_example' # str | 커서 동점 해결: 이전 응답에서 nextAfterUserId를 전달합니다. afterName이 설정된 경우 이름 동점이 항목을 누락하지 않도록 필요합니다. (옵션)

    try:
        api_response = api_instance.get_online_users(tenant_id, url_id, GetOnlineUsersOptions(after_name=after_name, after_user_id=after_user_id))
        print("The response of PublicApi->get_online_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_online_users: %s\n" % e)
[inline-code-end]