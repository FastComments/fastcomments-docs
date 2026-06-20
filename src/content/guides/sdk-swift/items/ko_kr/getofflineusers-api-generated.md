페이지에 과거에 댓글을 남겼지만 현재 온라인 상태가 아닌 사용자들입니다. displayName으로 정렬됩니다.
멤버 섹션을 렌더링하기 위해 /users/online을 모두 사용한 후 이것을 사용하세요.
commenterName에 대한 커서 페이지네이션: 서버는 부분 {tenantId, urlId, commenterName} 인덱스를 사용하여 afterName 이후부터 앞으로 $gt로 조회합니다. $skip 비용이 들지 않습니다.

## 매개변수

| 이름 | 유형 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Page URL identifier (cleaned server-side). |
| afterName | string | query | No | Cursor: pass nextAfterName from the previous response. |
| afterUserId | string | query | No | Cursor tiebreaker: pass nextAfterUserId from the previous response. Required when afterName is set so name-ties don't drop entries. |

## 응답

반환: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersOfflineResponse.swift)

## 예제

[inline-code-attrs-start title = 'getOfflineUsers 예제'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 다음 코드 샘플은 아직 베타입니다. 문제가 있을 경우 http://github.com/OpenAPITools/openapi-generator/issues/new 를 통해 신고하세요
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | Page URL identifier (cleaned server-side).
let afterName = "afterName_example" // String | Cursor: pass nextAfterName from the previous response. (optional)
let afterUserId = "afterUserId_example" // String | Cursor tiebreaker: pass nextAfterUserId from the previous response. Required when afterName is set so name-ties don't drop entries. (optional)

PublicAPI.getOfflineUsers(tenantId: tenantId, urlId: urlId, afterName: afterName, afterUserId: afterUserId) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]