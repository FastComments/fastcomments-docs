List pages for a tenant. Used by the FChat desktop client to populate its room list.  
Requires `enableFChat` to be true on the resolved custom config for each page.  
Pages that require SSO are filtered against the requesting user's group access.

## Parameters

| 이름 | 타입 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | 이전 요청에서 `nextCursor`로 반환된 불투명 페이지네이션 커서. 동일한 `sortBy`와 연결됩니다. |
| limit | integer | query | No | 1..200, 기본값 50 |
| q | string | query | No | 옵션 대소문자 구분 없는 제목 앞부분 필터. |
| sortBy | string | query | No | 정렬 순서. `updatedAt` (기본값, 최신 순), `commentCount` (댓글이 많은 순), 또는 `title` (알파벳 순). |
| hasComments | boolean | query | No | true인 경우, 최소 하나의 댓글이 있는 페이지만 반환합니다. |

## Response

반환: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPublicPagesResponse.swift)

## Example

[inline-code-attrs-start title = 'getPagesPublic 예시'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 다음 코드 샘플은 아직 베타 버전입니다. 문제가 있으면 http://github.com/OpenAPITools/openapi-generator/issues/new 를 통해 보고해 주세요
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let cursor = "cursor_example" // String | 이전 요청에서 `nextCursor`로 반환된 불투명 페이지네이션 커서. 동일한 `sortBy`와 연결됩니다. (옵션)
let limit = 987 // Int | 1..200, 기본값 50 (옵션)
let q = "q_example" // String | 옵션 대소문자 구분 없는 제목 앞부분 필터. (옵션)
let sortBy = PagesSortBy() // PagesSortBy | 정렬 순서. `updatedAt` (기본값, 최신 순), `commentCount` (댓글이 많은 순), 또는 `title` (알파벳 순). (옵션)
let hasComments = true // Bool | true인 경우, 최소 하나의 댓글이 있는 페이지만 반환합니다. (옵션)

PublicAPI.getPagesPublic(tenantId: tenantId, options: PublicAPI.GetPagesPublicOptions(cursor: cursor, limit: limit, q: q, sortBy: sortBy, hasComments: hasComments)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]