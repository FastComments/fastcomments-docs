List pages for a tenant. Used by the FChat desktop client to populate its room list.
Requires `enableFChat` to be true on the resolved custom config for each page.
Pages that require SSO are filtered against the requesting user's group access.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Необычный курсор пагинации, возвращённый как `nextCursor` из предыдущего запроса. Связан с тем же параметром `sortBy`. |
| limit | integer | query | No | 1..200, default 50 |
| q | string | query | No | Необязательный фильтр префикса названия без учёта регистра. |
| sortBy | string | query | No | Порядок сортировки. `updatedAt` (по умолчанию, от новых к старым), `commentCount` (со сначала страницы с наибольшим количеством комментариев), или `title` (по алфавиту). |
| hasComments | boolean | query | No | Если true, только возвращаются страницы, имеющие хотя бы один комментарий. |

## Response

Returns: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPublicPagesResponse.swift)

## Example

[inline-code-attrs-start title = 'Пример getPagesPublic'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следующие примеры кода находятся в бета-версии. При возникновении проблем, пожалуйста, сообщайте по адресу http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let cursor = "cursor_example" // String | Необычный курсор пагинации, возвращённый как `nextCursor` из предыдущего запроса. Связан с тем же параметром `sortBy`. (необязательно)
let limit = 987 // Int | 1..200, default 50 (необязательно)
let q = "q_example" // String | Необязательный фильтр префикса названия без учёта регистра. (необязательно)
let sortBy = PagesSortBy() // PagesSortBy | Порядок сортировки. `updatedAt` (по умолчанию, от новых к старым), `commentCount` (со сначала страницы с наибольшим количеством комментариев), или `title` (по алфавиту). (необязательно)
let hasComments = true // Bool | Если true, только возвращаются страницы, имеющие хотя бы один комментарий. (необязательно)

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