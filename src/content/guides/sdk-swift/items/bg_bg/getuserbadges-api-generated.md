## Параметри

| Име | Тип | Местоположение | Задължително | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| userId | string | query | Не |  |
| badgeId | string | query | Не |  |
| type | number | query | Не |  |
| displayedOnComments | boolean | query | Не |  |
| limit | number | query | Не |  |
| skip | number | query | Не |  |

## Отговор

Връща: [`GetUserBadges200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetUserBadges200Response.swift)

## Пример

[inline-code-attrs-start title = 'Пример за getUserBadges'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следните примери за код все още са в бета версия. При проблеми, моля докладвайте чрез http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let userId = "userId_example" // String |  (незадължително)
let badgeId = "badgeId_example" // String |  (незадължително)
let type = 987 // Double |  (незадължително)
let displayedOnComments = true // Bool |  (незадължително)
let limit = 987 // Double |  (незадължително)
let skip = 987 // Double |  (незадължително)

DefaultAPI.getUserBadges(tenantId: tenantId, userId: userId, badgeId: badgeId, type: type, displayedOnComments: displayedOnComments, limit: limit, skip: skip) { (response, error) in
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