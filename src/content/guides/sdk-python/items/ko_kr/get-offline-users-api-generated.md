현재 온라인 상태가 아닌, 해당 페이지의 과거 댓글 작성자입니다. displayName 기준으로 정렬됩니다.
"멤버" 섹션을 렌더링하기 위해 /users/online을 모두 소진한 후 이 엔드포인트를 사용하세요.
commenterName 기준 커서 페이지네이션: 서버는 부분 {tenantId, urlId, commenterName}
인덱스를 afterName부터 $gt로 순회하며, $skip 비용이 없습니다.

## 매개변수

| 이름 | 타입 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | path | 예 |  |
| urlId | string | query | 예 | 페이지 URL 식별자(서버 측에서 정리됨). |
| afterName | string | query | 아니요 | 커서: 이전 응답의 nextAfterName을 전달하세요. |
| afterUserId | string | query | 아니요 | 커서 타이브레이커: 이전 응답의 nextAfterUserId를 전달하세요. afterName이 설정된 경우 필수이며, 이름이 같은 항목이 누락되지 않도록 합니다. |

## 응답

반환값: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/page_users_offline_response.py)

## 예제

[inline-code-attrs-start title = 'get_offline_users 예제'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import GetOfflineUsersOptions
from client.models.page_users_offline_response import PageUsersOfflineResponse
from client.rest import ApiException
from pprint import pprint

# 호스트 정의는 선택 사항이며 기본값은 https://fastcomments.com입니다
# 지원되는 모든 구성 매개변수 목록은 configuration.py를 참조하세요.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# API 클라이언트 인스턴스와 함께 컨텍스트를 입력하세요
with client.ApiClient(configuration) as api_client:
    # API 클래스의 인스턴스를 생성합니다
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | 페이지 URL 식별자(서버 측에서 정리됨).
    after_name = 'after_name_example' # str | 커서: 이전 응답의 nextAfterName을 전달하세요. (optional)
    after_user_id = 'after_user_id_example' # str | 커서 타이브레이커: 이전 응답의 nextAfterUserId를 전달하세요. afterName이 설정된 경우 필수이며, 이름이 같은 항목이 누락되지 않도록 합니다. (optional)

    try:
        api_response = api_instance.get_offline_users(tenant_id, url_id, GetOfflineUsersOptions(after_name=after_name, after_user_id=after_user_id))
        print("The response of PublicApi->get_offline_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_offline_users: %s\n" % e)
[inline-code-end]