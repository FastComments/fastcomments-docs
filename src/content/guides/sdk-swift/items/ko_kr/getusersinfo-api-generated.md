---
테넌트에 대한 대량 사용자 정보. 주어진 userIds로부터 User / SSOUser의 표시 정보를 반환합니다.
댓글 위젯에서 presence 이벤트로 방금 나타난 사용자를 보강하는 데 사용됩니다.
페이지 컨텍스트 없음: 프라이버시는 일관되게 적용됩니다(비공개 프로필은 마스킹됩니다).

## Parameters

| 이름 | 형식 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | path | 예 |  |
| ids | string | query | 예 | 쉼표로 구분된 userIds. |

## Response

반환: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersInfoResponse.swift)

## 예제

[inline-code-attrs-start title = 'getUsersInfo 예제'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 다음 코드 샘플은 아직 베타입니다. 문제 발생 시 http://github.com/OpenAPITools/openapi-generator/issues/new를 통해 보고해 주세요
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let ids = "ids_example" // String | 쉼표로 구분된 userIds.

PublicAPI.getUsersInfo(tenantId: tenantId, ids: ids) { (response, error) in
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