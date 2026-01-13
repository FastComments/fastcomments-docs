## Параметри

| Име | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| id | string | path | Да |  |
| skip | number | query | Не |  |

## Одговор

Враћа: [`GetEmailTemplateRenderErrors200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetEmailTemplateRenderErrors200Response.swift)

## Пример

[inline-code-attrs-start title = 'getEmailTemplateRenderErrors Пример'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следећи примјери кода су још у бета фази. За било који проблем, пријавите га преко http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let id = "id_example" // String | 
let skip = 987 // Double |  (опционо)

DefaultAPI.getEmailTemplateRenderErrors(tenantId: tenantId, id: id, skip: skip) { (response, error) in
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