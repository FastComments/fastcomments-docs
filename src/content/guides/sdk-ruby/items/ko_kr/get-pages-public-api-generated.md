테넌트의 페이지를 나열합니다. FChat 데스크톱 클라이언트가 룸 목록을 채우기 위해 사용합니다. 각 페이지에 대해 해석된 커스텀 구성에서 `enableFChat`이 true여야 합니다. SSO가 필요한 페이지는 요청하는 사용자의 그룹 접근 권한과 비교하여 필터링됩니다.

## 매개변수

| 이름 | 유형 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | 이전 요청에서 `nextCursor`로 반환된 불투명한 페이지네이션 커서입니다. 동일한 `sortBy`에 묶여 있습니다. |
| limit | integer | query | No | 1..200, 기본값 50 |
| q | string | query | No | 선택적(대소문자 구분 없음) 제목 접두사 필터입니다. |
| sortBy | string | query | No | 정렬 순서. `updatedAt` (기본값, 최신 순), `commentCount` (댓글 수 많은 순), 또는 `title` (알파벳순). |
| hasComments | boolean | query | No | true인 경우, 댓글이 하나 이상 있는 페이지만 반환합니다. |

## 응답

반환: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_public_pages_response.rb)

## 예제

[inline-code-attrs-start title = 'get_pages_public 예제'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
opts = {
  cursor: 'cursor_example', # String | 이전 요청에서 `nextCursor`로 반환된 불투명한 페이지네이션 커서입니다. 동일한 `sortBy`에 묶여 있습니다.
  limit: 56, # Integer | 1..200, 기본값 50
  q: 'q_example', # String | 선택적(대소문자 구분 없음) 제목 접두사 필터입니다.
  sort_by: FastCommentsClient::PagesSortBy::UPDATED_AT, # PagesSortBy | 정렬 순서. `updatedAt` (기본값, 최신 순), `commentCount` (댓글 수 많은 순), 또는 `title` (알파벳순).
  has_comments: true # Boolean | true인 경우, 댓글이 하나 이상 있는 페이지만 반환합니다.
}

begin
  
  result = api_instance.get_pages_public(tenant_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_pages_public: #{e}"
end
[inline-code-end]