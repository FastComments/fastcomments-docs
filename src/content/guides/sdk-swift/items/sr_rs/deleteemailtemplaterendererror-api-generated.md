## Параметри

| Назив | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| id | string | path | Да |  |
| errorId | string | path | Да |  |

## Одговор

Враћа: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/APIEmptyResponse.swift)

## Пример

[inline-code-attrs-start title = 'deleteEmailTemplateRenderError Пример'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// У случају било каквог проблема, пријавите га преко http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let id = "id_example" // String | 
let errorId = "errorId_example" // String | 

DefaultAPI.deleteEmailTemplateRenderError(tenantId: tenantId, id: id, errorId: errorId) { (response, error) in
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