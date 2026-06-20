테넌트의 페이지 목록을 반환합니다. FChat 데스크톱 클라이언트가 룸 목록을 채우는 데 사용됩니다.
`enableFChat`가 각 페이지에 대한 확인된 커스텀 구성에서 true여야 합니다.
SSO를 필요로 하는 페이지는 요청 사용자의 그룹 접근 권한에 따라 필터링됩니다.

## 매개변수

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | 예 |  |
| cursor | string | query | 아니오 | 이전 요청에서 `nextCursor`로 반환된 불투명한 페이지네이션 커서입니다. 동일한 `sortBy`에 묶여 있습니다. |
| limit | integer | query | 아니오 | 1..200, 기본값 50 |
| q | string | query | 아니오 | 선택적 대소문자 구분 없는 제목 접두사 필터. |
| sortBy | string | query | 아니오 | 정렬 순서. `updatedAt` (기본값, 최신 순), `commentCount` (댓글 수가 많은 순), 또는 `title` (알파벳순). |
| hasComments | boolean | query | 아니오 | true인 경우, 적어도 하나의 댓글이 있는 페이지만 반환합니다. |

## 응답

반환: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_public_pages_response.py)

## 예제

[inline-code-attrs-start title = 'get_pages_public 예제'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_public_pages_response import GetPublicPagesResponse
from client.models.pages_sort_by import PagesSortBy
from client.rest import ApiException
from pprint import pprint

# 호스트 정의는 선택 사항이며 기본값은 https://fastcomments.com 입니다
# 지원되는 모든 구성 매개변수 목록은 configuration.py를 참조하세요.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# API 클라이언트 인스턴스와 함께 컨텍스트를 엽니다
with client.ApiClient(configuration) as api_client:
    # API 클래스의 인스턴스를 생성합니다
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    cursor = 'cursor_example' # str | 이전 요청에서 `nextCursor`로 반환된 불투명한 페이지네이션 커서입니다. 동일한 `sortBy`에 묶여 있습니다. (선택 사항)
    limit = 56 # int | 1..200, 기본값 50 (선택 사항)
    q = 'q_example' # str | 선택적 대소문자 구분 없는 제목 접두사 필터입니다. (선택 사항)
    sort_by = client.PagesSortBy() # PagesSortBy | 정렬 순서. `updatedAt` (기본값, 최신 순), `commentCount` (댓글 수가 많은 순), 또는 `title` (알파벳순). (선택 사항)
    has_comments = True # bool | true인 경우, 적어도 하나의 댓글이 있는 페이지만 반환합니다. (선택 사항)

    try:
        api_response = api_instance.get_pages_public(tenant_id, cursor=cursor, limit=limit, q=q, sort_by=sort_by, has_comments=has_comments)
        print("The response of PublicApi->get_pages_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_pages_public: %s\n" % e)
[inline-code-end]