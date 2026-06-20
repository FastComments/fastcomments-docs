## 매개변수

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| commentId | string | path | 예 |  |
| includeByUserIdAndEmail | boolean | query | 아니오 |  |
| includeByIP | boolean | query | 아니오 |  |
| includeByEmailDomain | boolean | query | 아니오 |  |
| sso | string | query | 아니오 |  |

## 응답

반환: [`PreBanSummary`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PreBanSummary.swift)

## 예제

[inline-code-attrs-start title = 'getPreBanSummary 예제'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 다음 코드 샘플은 아직 베타입니다. 문제 발생 시 http://github.com/OpenAPITools/openapi-generator/issues/new 을 통해 보고하세요
import FastCommentsSwift

let commentId = "commentId_example" // String | 
let includeByUserIdAndEmail = true // Bool |  (선택 사항)
let includeByIP = true // Bool |  (선택 사항)
let includeByEmailDomain = true // Bool |  (선택 사항)
let sso = "sso_example" // String |  (선택 사항)

ModerationAPI.getPreBanSummary(commentId: commentId, includeByUserIdAndEmail: includeByUserIdAndEmail, includeByIP: includeByIP, includeByEmailDomain: includeByEmailDomain, sso: sso) { (response, error) in
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