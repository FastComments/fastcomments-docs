## 매개변수

| 이름 | 형식 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| commentId | string | path | 예 |  |
| spam | boolean | query | 아니요 |  |
| permNotSpam | boolean | query | 아니요 |  |
| sso | string | query | 아니요 |  |

## 응답

반환: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/APIEmptyResponse.swift)

## 예제

[inline-code-attrs-start title = 'postSetCommentSpamStatus 예제'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 다음 코드 샘플은 아직 베타입니다. 문제가 있는 경우 http://github.com/OpenAPITools/openapi-generator/issues/new 으로 보고해 주세요
import FastCommentsSwift

let commentId = "commentId_example" // String | 
let spam = true // Bool |  (선택 사항)
let permNotSpam = true // Bool |  (선택 사항)
let sso = "sso_example" // String |  (선택 사항)

ModerationAPI.postSetCommentSpamStatus(commentId: commentId, spam: spam, permNotSpam: permNotSpam, sso: sso) { (response, error) in
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