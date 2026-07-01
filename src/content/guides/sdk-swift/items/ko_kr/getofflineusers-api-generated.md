Past commenters on the page who are NOT currently online. Sorted by displayName.  
페이지에 과거에 댓글을 남겼지만 현재 온라인이 아닌 댓글자들. displayName 기준으로 정렬됩니다.  
Use this after exhausting /users/online to render a "Members" section.  
/users/online을 모두 사용한 후 "Members" 섹션을 렌더링하기 위해 사용합니다.  
Cursor pagination on commenterName: server walks the partial {tenantId, urlId, commenterName}  
index from afterName forward via $gt, no $skip cost.  
commenterName에 대한 커서 페이지네이션: 서버는 부분 {tenantId, urlId, commenterName} 인덱스를 afterName 이후부터 $gt를 이용해 탐색하며, $skip 비용이 없습니다.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | 페이지 URL 식별자 (서버 측에서 정리됨). |
| afterName | string | query | No | 커서: 이전 응답의 nextAfterName을 전달합니다. |
| afterUserId | string | query | No | 커서 동점 해결: 이전 응답의 nextAfterUserId를 전달합니다. afterName이 설정된 경우 이름 동점으로 인한 항목 누락을 방지하기 위해 필요합니다. |

## Response

Returns: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersOfflineResponse.swift)

## Example

[inline-code-attrs-start title = 'getOfflineUsers 예시'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 다음 코드 샘플은 아직 베타 버전입니다. 문제가 있으면 http://github.com/OpenAPITools/openapi-generator/issues/new 에 보고해 주세요
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 페이지 URL 식별자 (서버 측에서 정리됨).
let afterName = "afterName_example" // String | 커서: 이전 응답의 nextAfterName을 전달합니다. (옵션)
let afterUserId = "afterUserId_example" // String | 커서 동점 해결: 이전 응답의 nextAfterUserId를 전달합니다. afterName이 설정된 경우 이름 동점으로 인한 항목 누락을 방지하기 위해 필요합니다. (옵션)

PublicAPI.getOfflineUsers(tenantId: tenantId, urlId: urlId, options: PublicAPI.GetOfflineUsersOptions(afterName: afterName, afterUserId: afterUserId)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]