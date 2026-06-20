## Параметры

| Имя | Тип | Location | Обязательно | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| yearNumber | number | query | Нет |  |
| monthNumber | number | query | Нет |  |
| dayNumber | number | query | Нет |  |
| skip | number | query | Нет |  |

## Ответ

Возвращает: [`GetTenantDailyUsagesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetTenantDailyUsagesResponse.swift)

## Пример

[inline-code-attrs-start title = 'Пример getTenantDailyUsages'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следующие примеры кода всё ещё в бета-версии. В случае проблем, пожалуйста, сообщите через http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let yearNumber = 987 // Double |  (необязательно)
let monthNumber = 987 // Double |  (необязательно)
let dayNumber = 987 // Double |  (необязательно)
let skip = 987 // Double |  (необязательно)

DefaultAPI.getTenantDailyUsages(tenantId: tenantId, yearNumber: yearNumber, monthNumber: monthNumber, dayNumber: dayNumber, skip: skip) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]