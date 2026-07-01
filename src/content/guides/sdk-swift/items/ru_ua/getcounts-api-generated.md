## Параметры

| Имя | Тип | Расположение | Обязательно | Описание |
|------|------|----------------|--------------|-----------|
| tenantId | string | query | Yes |  |
| sso | string | query | No |  |

## Ответ

Возвращает: [`GetBannedUsersCountResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetBannedUsersCountResponse.swift)

## Пример

[inline-code-attrs-start title = 'getCounts Пример'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следующий пример кода всё ещё в бета‑версии. При любой проблеме пожалуйста сообщите через http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let sso = "sso_example" // String |  (необязательно)

ModerationAPI.getCounts(tenantId: tenantId, sso: sso) { (response, error) in
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