Список страниц для тенанта. Используется настольным клиентом FChat для заполнения списка комнат.
Требуется, чтобы `enableFChat` имел значение true в итоговой (resolved) кастомной конфигурации для каждой страницы.
Страницы, требующие SSO, фильтруются в соответствии с групповым доступом запрашивающего пользователя.

## Параметры

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Непрозрачный курсор пагинации, возвращаемый как `nextCursor` в предыдущем запросе. Привязан к тому же `sortBy`. |
| limit | integer | query | No | 1..200, по умолчанию 50 |
| q | string | query | No | Необязательный фильтр по префиксу заголовка без учета регистра. |
| sortBy | string | query | No | Порядок сортировки. `updatedAt` (по умолчанию, сначала самые новые), `commentCount` (сначала страницы с наибольшим числом комментариев) или `title` (в алфавитном порядке). |
| hasComments | boolean | query | No | Если true, возвращать только страницы, имеющие хотя бы один комментарий. |

## Ответ

Возвращает: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPublicPagesResponse.swift)

## Пример

[inline-code-attrs-start title = 'Пример getPagesPublic'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следующие примеры кода все ещё в бета-версии. По любым проблемам, пожалуйста, сообщите через http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let cursor = "cursor_example" // String | Непрозрачный курсор пагинации, возвращаемый как `nextCursor` в предыдущем запросе. Привязан к тому же `sortBy`. (необязательно)
let limit = 987 // Int | 1..200, по умолчанию 50 (необязательно)
let q = "q_example" // String | Необязательный фильтр по префиксу заголовка без учета регистра. (необязательно)
let sortBy = PagesSortBy() // PagesSortBy | Порядок сортировки. `updatedAt` (по умолчанию, сначала самые новые), `commentCount` (сначала страницы с наибольшим числом комментариев) или `title` (в алфавитном порядке). (необязательно)
let hasComments = true // Bool | Если true, возвращать только страницы, имеющие хотя бы один комментарий. (необязательно)

PublicAPI.getPagesPublic(tenantId: tenantId, cursor: cursor, limit: limit, q: q, sortBy: sortBy, hasComments: hasComments) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]