List pages for a tenant. Used by the FChat desktop client to populate its room list.  
Requires `enableFChat` to be true on the resolved custom config for each page.  
Pages that require SSO are filtered against the requesting user's group access.

## Параметри

| Назва | Тип | Розташування | Обов'язково | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Непрозорий курсор пагінації, повернутий як `nextCursor` у попередньому запиті. Пов'язаний із тим самим `sortBy`. |
| limit | integer | query | No | 1..200, за замовчуванням 50 |
| q | string | query | No | Необов'язковий фільтр префікса назви без урахування регістру. |
| sortBy | string | query | No | Порядок сортування. `updatedAt` (за замовчуванням, новіші спочатку), `commentCount` (спочатку з найбільшою кількістю коментарів), або `title` (алфавітний). |
| hasComments | boolean | query | No | Якщо true, повернути лише сторінки, що мають хоча б один коментар. |

## Відповідь

Returns: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPublicPagesResponse.swift)

## Приклад

[inline-code-attrs-start title = 'getPagesPublic Приклад'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Наступні приклади коду ще в бета‑версії. У разі проблем, будь ласка, повідомляйте за адресою http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let cursor = "cursor_example" // String | Непрозорий курсор пагінації, повернутий як `nextCursor` у попередньому запиті. Пов'язаний із тим самим `sortBy`. (optional)
let limit = 987 // Int | 1..200, за замовчуванням 50 (optional)
let q = "q_example" // String | Необов'язковий фільтр префікса назви без урахування регістру. (optional)
let sortBy = PagesSortBy() // PagesSortBy | Порядок сортування. `updatedAt` (за замовчуванням, новіші спочатку), `commentCount` (спочатку з найбільшою кількістю коментарів), або `title` (алфавітний). (optional)
let hasComments = true // Bool | Якщо true, повернути лише сторінки, що мають хоча б один коментар. (optional)

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