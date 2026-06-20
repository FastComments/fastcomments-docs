## 매개변수

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| commentId | string | path | 예 |  |
| sso | string | query | 아니오 |  |

## 응답

반환: [`GetCommentTextResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetCommentTextResponse.swift)

## 예제

[inline-code-attrs-start title = 'getModerationCommentText 예제'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 다음 코드 샘플은 아직 베타입니다. 문제 발생 시 http://github.com/OpenAPITools/openapi-generator/issues/new 를 통해 보고해 주세요
import FastCommentsSwift

let commentId = "commentId_example" // String | 
let sso = "sso_example" // String |  (선택 사항)

ModerationAPI.getModerationCommentText(commentId: commentId, sso: sso) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]