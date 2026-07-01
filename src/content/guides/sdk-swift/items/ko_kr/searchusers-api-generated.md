## 매개변수

| 이름 | 유형 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | path | 예 |  |
| urlId | string | query | 예 |  |
| usernameStartsWith | string | query | 아니오 |  |
| mentionGroupIds | array | query | 아니오 |  |
| sso | string | query | 아니오 |  |
| searchSection | string | query | 아니오 |  |

## 응답

반환: [`SearchUsersResult`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/SearchUsersResult.swift)

## 예시

[inline-code-attrs-start title = 'searchUsers 예시'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 다음 코드 샘플은 아직 베타 버전입니다. 문제가 있으면 http://github.com/OpenAPITools/openapi-generator/issues/new 를 통해 보고해 주세요
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 
let usernameStartsWith = "usernameStartsWith_example" // String |  (옵션)
let mentionGroupIds = ["inner_example"] // [String] |  (옵션)
let sso = "sso_example" // String |  (옵션)
let searchSection = "searchSection_example" // String |  (옵션)

PublicAPI.searchUsers(tenantId: tenantId, urlId: urlId, options: PublicAPI.SearchUsersOptions(usernameStartsWith: usernameStartsWith, mentionGroupIds: mentionGroupIds, sso: sso, searchSection: searchSection)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]