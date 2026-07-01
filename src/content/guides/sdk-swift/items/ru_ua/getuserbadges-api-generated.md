## Параметры

| Имя | Тип | Местоположение | Обязательно | Описание |
|------|------|----------------|-------------|----------|
| tenantId | string | query | Да |  |
| userId | string | query | Нет |  |
| badgeId | string | query | Нет |  |
| type | number | query | Нет |  |
| displayedOnComments | boolean | query | Нет |  |
| limit | number | query | Нет |  |
| skip | number | query | Нет |  |

## Ответ

Возвращает: [`APIGetUserBadgesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/APIGetUserBadgesResponse.swift)

## Пример

[inline-code-attrs-start title = 'Пример getUserBadges'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следующие образцы кода всё ещё находятся в бета-версии. При возникновении проблем, пожалуйста, сообщайте через http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let userId = "userId_example" // String |  (опционально)
let badgeId = "badgeId_example" // String |  (опционально)
let type = 987 // Double |  (опционально)
let displayedOnComments = true // Bool |  (опционально)
let limit = 987 // Double |  (опционально)
let skip = 987 // Double |  (опционально)

DefaultAPI.getUserBadges(tenantId: tenantId, options: DefaultAPI.GetUserBadgesOptions(userId: userId, badgeId: badgeId, type: type, displayedOnComments: displayedOnComments, limit: limit, skip: skip)) { (response, error) in
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