Who voted for which option. Anonymous polls never answer, admins-only polls answer only for an admin or moderator.

## Parameters

| 이름 | 유형 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | path | 예 |  |
| commentId | string | path | 예 |  |
| urlId | string | query | 아니오 |  |
| page | integer | query | 아니오 |  |
| search | string | query | 아니오 |  |
| sso | string | query | 아니오 |  |

## 응답

반환: [`GetPollVoterNamesSuccessResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPollVoterNamesSuccessResponse.swift)

## 예시

[inline-code-attrs-start title = 'getPollVoters 예시'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 다음 코드 샘플은 아직 베타 버전입니다. 문제가 있으면 http://github.com/OpenAPITools/openapi-generator/issues/new 로 보고해 주세요
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String | 
let urlId = "urlId_example" // String |  (옵션)
let page = 987 // Int |  (옵션)
let search = "search_example" // String |  (옵션)
let sso = "sso_example" // String |  (옵션)

PublicAPI.getPollVoters(tenantId: tenantId, commentId: commentId, options: PublicAPI.GetPollVotersOptions(urlId: urlId, page: page, search: search, sso: sso)) { (response, error) in
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