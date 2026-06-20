Перелічує сторінки для орендаря. Використовується десктоп-клієнтом FChat для заповнення списку кімнат.
Потребує, щоб `enableFChat` було true у розв'язаній кастомній конфігурації для кожної сторінки.
Сторінки, які вимагають SSO, фільтруються відповідно до групового доступу користувача, який робить запит.

## Параметри

| Назва | Тип | Розташування | Обов'язковий | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Непрозорий курсор пагінації, що повертається як `nextCursor` з попереднього запиту. Прив'язаний до того ж `sortBy`. |
| limit | integer | query | No | 1..200, за замовчуванням 50 |
| q | string | query | No | Необов'язковий фільтр префікса заголовка без врахування регістру. |
| sortBy | string | query | No | Порядок сортування. `updatedAt` (за замовчуванням, від найновіших), `commentCount` (спочатку сторінки з найбільшою кількістю коментарів), або `title` (алфавітно). |
| hasComments | boolean | query | No | Якщо true, повертати лише сторінки з принаймні одним коментарем. |

## Відповідь

Повертає: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPublicPagesResponse.swift)

## Приклад

[inline-code-attrs-start title = 'Приклад getPagesPublic'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Наведені приклади коду ще у бета-версії. У разі проблем, будь ласка, повідомте через http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let cursor = "cursor_example" // String | Непрозорий курсор пагінації, що повертається як `nextCursor` з попереднього запиту. Прив'язаний до того ж `sortBy`. (необов'язково)
let limit = 987 // Int | 1..200, за замовчуванням 50 (необов'язково)
let q = "q_example" // String | Необов'язковий фільтр префікса заголовка без врахування регістру. (необов'язково)
let sortBy = PagesSortBy() // PagesSortBy | Порядок сортування. `updatedAt` (за замовчуванням, від найновіших), `commentCount` (спочатку сторінки з найбільшою кількістю коментарів) або `title` (алфавітно). (необов'язково)
let hasComments = true // Bool | Якщо true, повертати лише сторінки з принаймні одним коментарем. (необов'язково)

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