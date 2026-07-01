List pages for a tenant. Used by the FChat desktop client to populate its room list.  
Requires `enableFChat` to be true on the resolved custom config for each page.  
Pages that require SSO are filtered against the requesting user's group access.

## Параметры

| Имя | Тип | Местоположение | Обязательно | Описание |
|------|------|----------------|-------------|----------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Неявный курсор пагинации, возвращённый как `nextCursor` из предыдущего запроса. Связан с тем же `sortBy`. |
| limit | integer | query | No | 1..200, default 50 |
| q | string | query | No | Необязательный фильтр по префиксу названия без учёта регистра. |
| sortBy | string | query | No | Порядок сортировки. `updatedAt` (по умолчанию, новые первыми), `commentCount` (сообщений больше сначала), или `title` (в алфавитном порядке). |
| hasComments | boolean | query | No | Если true, возвращать только страницы с хотя бы одним комментарием. |

## Ответ

Returns: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPublicPagesResponse.swift)

## Пример

[inline-code-attrs-start title = 'getPagesPublic Пример'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Последующие примеры кода находятся в бете. При возникновении проблем, пожалуйста, сообщайте по адресу http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let cursor = "cursor_example" // String | Неявный курсор пагинации, возвращённый как `nextCursor` из предыдущего запроса. Связан с тем же `sortBy`. (optional)
let limit = 987 // Int | 1..200, по умолчанию 50 (optional)
let q = "q_example" // String | Необязательный фильтр по префиксу названия без учёта регистра. (optional)
let sortBy = PagesSortBy() // PagesSortBy | Порядок сортировки. `updatedAt` (по умолчанию, новые первыми), `commentCount` (сообщений больше сначала), или `title` (в алфавитном порядке). (optional)
let hasComments = true // Bool | Если true, возвращать только страницы с хотя бы одним комментарием. (optional)

PublicAPI.getPagesPublic(tenantId: tenantId, options: PublicAPI.GetPagesPublicOptions(cursor: cursor, limit: limit, q: q, sortBy: sortBy, hasComments: hasComments)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]