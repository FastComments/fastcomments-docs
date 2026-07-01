현재 페이지에 온라인으로 있는 시청자: 현재 웹소켓 세션이 해당 페이지에 구독된 사람들.  
익명 카운트와 총 카운트(방 전체 구독자, 우리가 열거하지 않는 익명 시청자 포함)를 반환합니다.

## Parameters

| 이름 | 유형 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | path | 예 |  |
| urlId | string | query | 예 | 페이지 URL 식별자 (서버 측에서 정리됨). |
| afterName | string | query | 아니오 | 커서: 이전 응답에서 nextAfterName을 전달합니다. |
| afterUserId | string | query | 아니오 | 커서 동점 해소: 이전 응답에서 nextAfterUserId를 전달합니다. afterName이 설정된 경우 이름 동점으로 인한 항목 손실을 방지하기 위해 필요합니다. |

## Response

반환: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersOnlineResponse.swift)

## 예시

[inline-code-attrs-start title = 'getOnlineUsers 예시'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 다음 코드 샘플은 아직 베타 단계입니다. 문제가 있으면 http://github.com/OpenAPITools/openapi-generator/issues/new 를 통해 보고해 주세요
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 페이지 URL 식별자 (서버 측에서 정리됨).
let afterName = "afterName_example" // String | 커서: 이전 응답에서 nextAfterName을 전달합니다. (선택 사항)
let afterUserId = "afterUserId_example" // String | 커서 동점 해소: 이전 응답에서 nextAfterUserId를 전달합니다. afterName이 설정된 경우 이름 동점으로 인한 항목 손실을 방지하기 위해 필요합니다. (선택 사항)

PublicAPI.getOnlineUsers(tenantId: tenantId, urlId: urlId, options: PublicAPI.GetOnlineUsersOptions(afterName: afterName, afterUserId: afterUserId)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]