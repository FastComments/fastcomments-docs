## Параметри

| Назив | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| meta | string | query | Не |  |
| skip | number | query | Не |  |

## Одговор

Враћа: [`GetTenants200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetTenants200Response.swift)

## Пример

[inline-code-attrs-start title = 'getTenants Пример'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следећи примери кода су још у бети. За било који проблем, пријавите га путем http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let meta = "meta_example" // String |  (опционо)
let skip = 987 // Double |  (опционо)

DefaultAPI.getTenants(tenantId: tenantId, meta: meta, skip: skip) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]