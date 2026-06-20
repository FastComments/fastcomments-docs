Пакетна информация за потребители за наемател. При зададени userIds, връща информация за показване от User / SSOUser.
Използва се от коментарния widget за обогатяване на потребители, които току-що се появиха чрез събитие за присъствие.
Без контекст на страница: поверителността се прилага еднакво (частните профили са маскирани).

## Параметри

| Име | Тип | Местоположение | Задължително | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| ids | string | query | Да | userIds, разделени със запетая. |

## Отговор

Връща: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersInfoResponse.swift)

## Пример

[inline-code-attrs-start title = 'Пример за getUsersInfo'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следващите примери за код все още са в бета. За проблеми, моля докладвайте чрез http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let ids = "ids_example" // String | userIds, разделени със запетая.

PublicAPI.getUsersInfo(tenantId: tenantId, ids: ids) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]