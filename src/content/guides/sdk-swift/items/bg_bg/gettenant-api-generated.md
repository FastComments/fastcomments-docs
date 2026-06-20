## Параметри

| Name | Type | Location | Задължително | Description |
|------|------|----------|---------------|-------------|
| tenantId | string | query | Да |  |
| id | string | path | Да |  |

## Отговор

Връща: [`GetTenantResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetTenantResponse.swift)

## Пример

[inline-code-attrs-start title = 'getTenant пример'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следните примерни фрагменти от код все още са в бета. За всеки проблем, моля, докладвайте чрез http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let id = "id_example" // String | 

DefaultAPI.getTenant(tenantId: tenantId, id: id) { (response, error) in
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