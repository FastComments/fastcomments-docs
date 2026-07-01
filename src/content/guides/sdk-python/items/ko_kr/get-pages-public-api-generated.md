List pages for a tenant. Used by the FChat desktop client to populate its room list.
Requires `enableFChat` to be true on the resolved custom config for each page.
Pages that require SSO are filtered against the requesting user's group access.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | 이전 요청에서 `nextCursor` 로 반환된 불투명 페이지네이션 커서입니다. 동일한 `sortBy` 와 연결됩니다. |
| limit | integer | query | No | 1..200, 기본값 50 |
| q | string | query | No | 옵션인 대소문자 구분 없는 제목 접두사 필터입니다. |
| sortBy | string | query | No | 정렬 순서. `updatedAt` (기본값, 최신 순), `commentCount` (댓글이 가장 많은 순), 또는 `title` (알파벳 순). |
| hasComments | boolean | query | No | true인 경우, 최소 하나의 댓글이 있는 페이지만 반환합니다. |

## Response

Returns: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_public_pages_response.py)

## 예제

[inline-code-attrs-start title = 'get_pages_public 예제'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import GetPagesPublicOptions
from client.models.get_public_pages_response import GetPublicPagesResponse
from client.models.pages_sort_by import PagesSortBy
from client.rest import ApiException
from pprint import pprint

# 호스트를 정의하는 것은 선택 사항이며 기본값은 https://fastcomments.com 입니다
# configuration.py에서 지원되는 모든 구성 매개변수 목록을 확인하세요.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Enter a context with an instance of the API client
with client.ApiClient(configuration) as api_client:
    # API 클래스의 인스턴스를 생성합니다
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    cursor = 'cursor_example' # str | 이전 요청에서 `nextCursor` 로 반환된 불투명 페이지네이션 커서입니다. 동일한 `sortBy` 와 연결됩니다. (선택 사항)
    limit = 56 # int | 1..200, 기본값 50 (선택 사항)
    q = 'q_example' # str | 옵션인 대소문자 구분 없는 제목 접두사 필터입니다. (선택 사항)
    sort_by = client.PagesSortBy() # PagesSortBy | 정렬 순서. `updatedAt` (기본값, 최신 순), `commentCount` (댓글이 가장 많은 순), 또는 `title` (알파벳 순). (선택 사항)
    has_comments = True # bool | true인 경우, 최소 하나의 댓글이 있는 페이지만 반환합니다. (선택 사항)

    try:
        api_response = api_instance.get_pages_public(tenant_id, GetPagesPublicOptions(cursor=cursor, limit=limit, q=q, sort_by=sort_by, has_comments=has_comments))
        print("The response of PublicApi->get_pages_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_pages_public: %s\n" % e)
[inline-code-end]