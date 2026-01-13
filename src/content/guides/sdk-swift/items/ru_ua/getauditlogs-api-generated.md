## Параметры

| Имя | Тип | Расположение | Обязательно | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| limit | number | query | Нет |  |
| skip | number | query | Нет |  |
| order | string | query | Нет |  |
| after | number | query | Нет |  |
| before | number | query | Нет |  |

## Ответ

Возвращает: [`GetAuditLogs200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetAuditLogs200Response.swift)

## Пример

[inline-code-attrs-start title = 'Пример getAuditLogs'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следующие примеры кода все ещё находятся в бета‑версии. Если возникли проблемы, пожалуйста, сообщите через http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let limit = 987 // Double |  (необязательно)
let skip = 987 // Double |  (необязательно)
let order = SORT_DIR() // SORTDIR |  (необязательно)
let after = 987 // Double |  (необязательно)
let before = 987 // Double |  (необязательно)

DefaultAPI.getAuditLogs(tenantId: tenantId, limit: limit, skip: skip, order: order, after: after, before: before) { (response, error) in
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