## 매개변수

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | 예 |  |
| postId | string | path | 예 |  |
| isUndo | boolean | query | 아니오 |  |
| broadcastId | string | query | 아니오 |  |
| sso | string | query | 아니오 |  |

## 응답

반환: [`ReactFeedPostResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/ReactFeedPostResponse.swift)

## 예제

[inline-code-attrs-start title = 'reactFeedPostPublic 예제'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 다음 코드 샘플은 아직 베타입니다. 문제가 있으면 http://github.com/OpenAPITools/openapi-generator/issues/new 을 통해 보고해 주세요
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let postId = "postId_example" // String | 
let reactBodyParams = ReactBodyParams(reactType: "reactType_example") // ReactBodyParams | 
let isUndo = true // Bool |  (선택 사항)
let broadcastId = "broadcastId_example" // String |  (선택 사항)
let sso = "sso_example" // String |  (선택 사항)

PublicAPI.reactFeedPostPublic(tenantId: tenantId, postId: postId, reactBodyParams: reactBodyParams, isUndo: isUndo, broadcastId: broadcastId, sso: sso) { (response, error) in
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