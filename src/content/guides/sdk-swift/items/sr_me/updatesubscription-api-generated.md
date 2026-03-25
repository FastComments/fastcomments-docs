## Параметри

| Назив | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| id | string | path | Да |  |
| userId | string | query | Не |  |

## Одговор

Враћа: [`UpdateSubscriptionAPIResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/UpdateSubscriptionAPIResponse.swift)

## Пример

[inline-code-attrs-start title = 'updateSubscription Пример'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следећи примјери кода су још у бета фази. За било који проблем, молимо пријавите на http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let id = "id_example" // String | 
let updateAPIUserSubscriptionData = UpdateAPIUserSubscriptionData(notificationFrequency: 123) // UpdateAPIUserSubscriptionData | 
let userId = "userId_example" // String |  (опционо)

DefaultAPI.updateSubscription(tenantId: tenantId, id: id, updateAPIUserSubscriptionData: updateAPIUserSubscriptionData, userId: userId) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]