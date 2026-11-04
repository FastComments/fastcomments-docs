테넌트의 페이지 목록을 조회합니다. FChat 데스크톱 클라이언트가 방 목록을 채우는 데 사용됩니다.
각 페이지의 확인된 커스텀 설정에서 `enableFChat`이 true여야 합니다.
SSO가 필요한 페이지는 요청 사용자의 그룹 접근 권한을 기준으로 필터링됩니다.

## 매개변수

| 이름 | 타입 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | path | 예 |  |
| cursor | string | query | 아니요 | 이전 요청에서 `nextCursor`로 반환된 불투명한 페이지네이션 커서입니다. 동일한 `sortBy`에 연결됩니다. |
| limit | integer | query | 아니요 | 1..200, 기본값 50 |
| q | string | query | 아니요 | 선택적 대소문자를 구분하지 않는 제목 접두사 필터입니다. |
| sortBy | string | query | 아니요 | 정렬 순서. `updatedAt` (기본값, 최신순), `commentCount` (댓글 많은 순), 또는 `title` (알파벳순). |
| hasComments | boolean | query | 아니요 | true이면 댓글이 하나 이상 있는 페이지만 반환합니다. |

## 응답

반환값: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_public_pages_response.py)

## 예제

[inline-code-attrs-start title = 'get_pages_public 예제'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import GetPagesPublicOptions
from client.models.get_public_pages_response import GetPublicPagesResponse
from client.models.pages_sort_by import PagesSortBy
from client.rest import ApiException
from pprint import pprint

# 호스트를 정의하는 것은 선택 사항이며 기본값은 https://fastcomments.com입니다
# 지원되는 모든 구성 매개변수 목록은 configuration.py를 참조하세요.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# API 클라이언트 인스턴스가 있는 컨텍스트에 진입합니다
with client.ApiClient(configuration) as api_client:
    # API 클래스의 인스턴스를 생성합니다
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    cursor = 'cursor_example' # str | 이전 요청에서 `nextCursor`로 반환된 불투명한 페이지네이션 커서입니다. 동일한 `sortBy`에 연결됩니다. (optional)
    limit = 56 # int | 1..200, 기본값 50 (optional)
    q = 'q_example' # str | 선택적 대소문자를 구분하지 않는 제목 접두사 필터입니다. (optional)
    sort_by = client.PagesSortBy() # PagesSortBy | 정렬 순서. `updatedAt` (기본값, 최신순), `commentCount` (댓글 많은 순), 또는 `title` (알파벳순). (optional)
    has_comments = True # bool | true이면 댓글이 하나 이상 있는 페이지만 반환합니다. (optional)

    try:
        api_response = api_instance.get_pages_public(tenant_id, GetPagesPublicOptions(cursor=cursor, limit=limit, q=q, sort_by=sort_by, has_comments=has_comments))
        print("The response of PublicApi->get_pages_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_pages_public: %s\n" % e)
[inline-code-end]