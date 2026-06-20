테넌트의 페이지 목록을 나열합니다. FChat 데스크탑 클라이언트에서 룸 목록을 채우는 데 사용됩니다.
각 페이지에 대해 해결된 커스텀 구성에서 `enableFChat`이 true여야 합니다.
SSO가 필요한 페이지는 요청한 사용자의 그룹 접근 권한에 따라 필터링됩니다.

## 매개변수

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | 예 |  |
| cursor | string | query | 아니오 | 이전 요청에서 `nextCursor`로 반환된 불투명한 페이지네이션 커서입니다. 동일한 `sortBy`와 연결됩니다. |
| limit | integer | query | 아니오 | 1..200, 기본값 50 |
| q | string | query | 아니오 | 선택 사항인 대소문자 구분 없는 제목 접두사 필터. |
| sortBy | string | query | 아니오 | 정렬 순서. `updatedAt` (기본값, 최신순), `commentCount` (댓글 많은 순), 또는 `title` (알파벳순). |
| hasComments | boolean | query | 아니오 | true인 경우, 댓글이 하나 이상 있는 페이지만 반환합니다. |

## 응답

반환: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPublicPagesResponse.swift)

## 예제

[inline-code-attrs-start title = 'getPagesPublic 예제'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 다음 코드 샘플은 아직 베타입니다. 문제 발생 시 http://github.com/OpenAPITools/openapi-generator/issues/new를 통해 보고해 주세요
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let cursor = "cursor_example" // String | 이전 요청에서 `nextCursor`로 반환된 불투명한 페이지네이션 커서입니다. 동일한 `sortBy`에 묶입니다. (선택 사항)
let limit = 987 // Int | 1..200, 기본값 50 (선택 사항)
let q = "q_example" // String | 선택 사항인 대소문자 구분 없는 제목 접두사 필터입니다. (선택 사항)
let sortBy = PagesSortBy() // PagesSortBy | 정렬 순서. `updatedAt` (기본값, 최신순), `commentCount` (댓글 많은 순), 또는 `title` (알파벳순). (선택 사항)
let hasComments = true // Bool | true인 경우, 댓글이 하나 이상 있는 페이지만 반환합니다. (선택 사항)

PublicAPI.getPagesPublic(tenantId: tenantId, cursor: cursor, limit: limit, q: q, sortBy: sortBy, hasComments: hasComments) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]