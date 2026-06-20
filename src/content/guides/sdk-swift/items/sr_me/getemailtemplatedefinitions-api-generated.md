## Параметри

| Назив | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |

## Одговор

Враћа: [`GetEmailTemplateDefinitionsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetEmailTemplateDefinitionsResponse.swift)

## Примјер

[inline-code-attrs-start title = 'getEmailTemplateDefinitions Примјер'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Сљедећи примјери кода су још у бета фази. За било који проблем, пријавите путем http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 

DefaultAPI.getEmailTemplateDefinitions(tenantId: tenantId) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]