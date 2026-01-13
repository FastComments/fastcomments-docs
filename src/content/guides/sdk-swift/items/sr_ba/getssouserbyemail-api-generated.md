## Параметри

| Име | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| email | string | path | Yes |  |

## Одговор

Враћа: [`GetSSOUserByEmailAPIResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetSSOUserByEmailAPIResponse.swift)

## Пример

[inline-code-attrs-start title = 'getSSOUserByEmail Пример'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Сљедећи примјери кода су још у бета фази. За било који проблем пријавите га преко http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let email = "email_example" // String | 

DefaultAPI.getSSOUserByEmail(tenantId: tenantId, email: email) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]