## Параметры

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| urlId | string | query | Да |  |
| usernameStartsWith | string | query | Нет |  |
| mentionGroupIds | array | query | Нет |  |
| sso | string | query | Нет |  |
| searchSection | string | query | Нет |  |

## Ответ

Возвращает: [`SearchUsers200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/SearchUsers200Response.swift)

## Пример

[inline-code-attrs-start title = 'Пример searchUsers'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следующие примеры кода всё ещё в бета-версии. Для любых проблем, пожалуйста, сообщите через http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 
let usernameStartsWith = "usernameStartsWith_example" // String |  (необязательно)
let mentionGroupIds = ["inner_example"] // [String] |  (необязательно)
let sso = "sso_example" // String |  (необязательно)
let searchSection = "searchSection_example" // String |  (необязательно)

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