Перелічує сторінки для тенанта. Використовується десктопним клієнтом FChat для заповнення списку кімнат.
Потребує `enableFChat` = true у вирішеному кастомному конфізі для кожної сторінки.
Сторінки, що потребують SSO, фільтруються відповідно до групового доступу користувача, що робить запит.

## Параметри

| Назва | Тип | Розташування | Обов'язковий | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | path | Так |  |
| cursor | string | query | Ні | Непрозорий курсор пагінації, повернутий як `nextCursor` з попереднього запиту. Прив'язаний до того ж `sortBy`. |
| limit | integer | query | Ні | 1..200, за замовчуванням 50 |
| q | string | query | Ні | Необов'язковий фільтр за префіксом назви без урахування регістру. |
| sortBy | string | query | Ні | Порядок сортування. `updatedAt` (за замовчуванням, спочатку новіші), `commentCount` (спочатку з найбільшою кількістю коментарів) або `title` (за алфавітом). |
| hasComments | boolean | query | Ні | Якщо true, повертати лише сторінки з принаймні одним коментарем. |

## Відповідь

Повертає: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPublicPagesResponse.swift)

## Приклад

[inline-code-attrs-start title = 'Приклад getPagesPublic'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Наступні приклади коду все ще в бета-версії. Про будь-які проблеми повідомляйте на http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let cursor = "cursor_example" // String | Непрозорий курсор пагінації, повернутий як `nextCursor` з попереднього запиту. Прив'язаний до того ж `sortBy`. (optional)
let limit = 987 // Int | 1..200, за замовчуванням 50 (optional)
let q = "q_example" // String | Необов'язковий фільтр за префіксом назви без урахування регістру. (optional)
let sortBy = PagesSortBy() // PagesSortBy | Порядок сортування. `updatedAt` (за замовчуванням, спочатку новіші), `commentCount` (спочатку з найбільшою кількістю коментарів) або `title` (за алфавітом). (optional)
let hasComments = true // Bool | Якщо true, повертати лише сторінки з принаймні одним коментарем. (optional)

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