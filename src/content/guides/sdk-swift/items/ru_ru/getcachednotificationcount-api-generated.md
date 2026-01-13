## Параметры

| Имя | Тип | Расположение | Обязательно | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| id | string | path | Да |  |

## Ответ

Возвращает: [`GetCachedNotificationCount200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetCachedNotificationCount200Response.swift)

## Пример

[inline-code-attrs-start title = 'Пример getCachedNotificationCount'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Приведённые ниже примеры кода всё ещё находятся в бета-версии. По любым проблемам, пожалуйста, сообщайте через http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let id = "id_example" // String | 

DefaultAPI.getCachedNotificationCount(tenantId: tenantId, id: id) { (response, error) in
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