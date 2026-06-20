---
페이지의 현재 온라인 시청자: 현재 websocket 세션이 해당 페이지에 구독 중인 사람들. anonCount + totalCount를 반환합니다 (룸 전체 구독자, 우리가 열거하지 않는 anon viewers 포함).

## 매개변수

| 이름 | 유형 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | 경로 | 예 |  |
| urlId | string | 쿼리 | 예 | 페이지 URL 식별자 (서버 측에서 정리됨). |
| afterName | string | 쿼리 | 아니오 | 커서: 이전 응답의 nextAfterName을 전달하세요. |
| afterUserId | string | 쿼리 | 아니오 | 커서 타이브레이커: 이전 응답의 nextAfterUserId를 전달하세요. afterName이 설정된 경우 이름 동률로 인해 항목이 누락되지 않도록 필요합니다. |

## 응답

반환: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersOnlineResponse.swift)

## 예제

[inline-code-attrs-start title = 'getOnlineUsers 예제'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 다음 코드 샘플은 아직 베타입니다. 문제 발생 시 http://github.com/OpenAPITools/openapi-generator/issues/new 를 통해 신고해 주세요
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 페이지 URL 식별자 (서버 측에서 정리됨).
let afterName = "afterName_example" // String | 커서: 이전 응답의 nextAfterName을 전달하세요. (선택 사항)
let afterUserId = "afterUserId_example" // String | 커서 타이브레이커: 이전 응답의 nextAfterUserId를 전달하세요. afterName이 설정된 경우 이름 동률로 인해 항목이 누락되지 않도록 필요합니다. (선택 사항)

PublicAPI.getOnlineUsers(tenantId: tenantId, urlId: urlId, afterName: afterName, afterUserId: afterUserId) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]

---