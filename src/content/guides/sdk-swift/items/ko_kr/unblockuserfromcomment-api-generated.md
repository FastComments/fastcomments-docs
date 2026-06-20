## 매개변수

| 이름 | Type | Location | Required | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 예 |  |
| id | string | path | 예 |  |
| userId | string | query | 아니오 |  |
| anonUserId | string | query | 아니오 |  |

## 응답

반환: [`UnblockSuccess`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/UnblockSuccess.swift)

## 예제

[inline-code-attrs-start title = 'unBlockUserFromComment 예제'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 다음 코드 샘플은 아직 베타입니다. 문제 발생 시 http://github.com/OpenAPITools/openapi-generator/issues/new 를 통해 보고해 주세요
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let id = "id_example" // String | 
let unBlockFromCommentParams = UnBlockFromCommentParams(commentIdsToCheck: ["commentIdsToCheck_example"]) // UnBlockFromCommentParams | 
let userId = "userId_example" // String |  (선택 사항)
let anonUserId = "anonUserId_example" // String |  (선택 사항)

DefaultAPI.unBlockUserFromComment(tenantId: tenantId, id: id, unBlockFromCommentParams: unBlockFromCommentParams, userId: userId, anonUserId: anonUserId) { (response, error) in
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