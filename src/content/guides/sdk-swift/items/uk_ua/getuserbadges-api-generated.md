## Параметри

| Назва | Тип | Розташування | Обов'язково | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Так |  |
| userId | string | query | Ні |  |
| badgeId | string | query | Ні |  |
| type | number | query | Ні |  |
| displayedOnComments | boolean | query | Ні |  |
| limit | number | query | Ні |  |
| skip | number | query | Ні |  |

## Відповідь

Повертає: [`GetUserBadges200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetUserBadges200Response.swift)

## Приклад

[inline-code-attrs-start title = 'Приклад getUserBadges'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Наступні приклади коду все ще в бета-версії. У разі будь-якої проблеми, будь ласка, повідомте через http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let userId = "userId_example" // String |  (необов'язково)
let badgeId = "badgeId_example" // String |  (необов'язково)
let type = 987 // Double |  (необов'язково)
let displayedOnComments = true // Bool |  (необов'язково)
let limit = 987 // Double |  (необов'язково)
let skip = 987 // Double |  (необов'язково)

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