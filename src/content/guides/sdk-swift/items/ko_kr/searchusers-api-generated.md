## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | 예 |  |
| urlId | string | query | 예 |  |
| usernameStartsWith | string | query | 아니요 |  |
| mentionGroupIds | array | query | 아니요 |  |
| sso | string | query | 아니요 |  |
| searchSection | string | query | 아니요 |  |

## Response

반환: [`SearchUsersResult`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/SearchUsersResult.swift)

## Example

[inline-code-attrs-start title = 'searchUsers 예제'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 다음 코드 샘플은 아직 베타입니다. 문제 발생 시 http://github.com/OpenAPITools/openapi-generator/issues/new 를 통해 보고해 주세요
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 
let usernameStartsWith = "usernameStartsWith_example" // String |  (선택 사항)
let mentionGroupIds = ["inner_example"] // [String] |  (선택 사항)
let sso = "sso_example" // String |  (선택 사항)
let searchSection = "searchSection_example" // String |  (선택 사항)

PublicAPI.searchUsers(tenantId: tenantId, urlId: urlId, usernameStartsWith: usernameStartsWith, mentionGroupIds: mentionGroupIds, sso: sso, searchSection: searchSection) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]