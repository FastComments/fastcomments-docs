## Параметри

| Име | Тип | Локация | Задължително | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| urlIdWS | string | query | Да |  |
| userIds | string | query | Да |  |

## Отговор

Връща: [`GetUserPresenceStatuses200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetUserPresenceStatuses200Response.swift)

## Пример

[inline-code-attrs-start title = 'Пример за getUserPresenceStatuses'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следните примери за код все още са в бета. За всеки проблем, моля докладвайте чрез http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlIdWS = "urlIdWS_example" // String | 
let userIds = "userIds_example" // String | 

PublicAPI.getUserPresenceStatuses(tenantId: tenantId, urlIdWS: urlIdWS, userIds: userIds) { (response, error) in
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