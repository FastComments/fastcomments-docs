## Параметри

| Назва | Тип | Розташування | Обов’язковий | Опис |
|------|------|--------------|--------------|------|
| tenantId | string | path | Так |  |
| urlId | string | query | Так |  |
| usernameStartsWith | string | query | Ні |  |
| mentionGroupIds | array | query | Ні |  |
| sso | string | query | Ні |  |
| searchSection | string | query | Ні |  |

## Відповідь

Повертає: [`SearchUsersResult`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/SearchUsersResult.swift)

## Приклад

[inline-code-attrs-start title = 'searchUsers Приклад'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Наступні зразки коду ще у бета-версії. У разі проблем, будь ласка, повідомте за адресою http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 
let usernameStartsWith = "usernameStartsWith_example" // String |  (необов’язково)
let mentionGroupIds = ["inner_example"] // [String] |  (необов’язково)
let sso = "sso_example" // String |  (необов’язково)
let searchSection = "searchSection_example" // String |  (необов’язково)

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