Изброява страници за наемател. Използва се от десктоп клиента FChat за попълване на списъка му със стаи.
Изисква `enableFChat` да е true в резултантната персонализирана конфигурация за всяка страница.
Страниците, които изискват SSO, се филтрират спрямо груповия достъп на потребителя, правещ заявката.

## Параметри

| Име | Тип | Местоположение | Задължително | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| cursor | string | query | Не | Непрозрачен курсор за странициране, върнат като `nextCursor` от предишна заявка. Свързан е със същия `sortBy`. |
| limit | integer | query | Не | 1..200, по подразбиране 50 |
| q | string | query | Не | Незадължителен регистронезависим филтър по префикс на заглавие. |
| sortBy | string | query | Не | Подредба. `updatedAt` (по подразбиране, най-новите първи), `commentCount` (най-много коментари първи), или `title` (азбучно). |
| hasComments | boolean | query | Не | Ако е true, връща само страници с поне един коментар. |

## Отговор

Връща: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPublicPagesResponse.swift)

## Пример

[inline-code-attrs-start title = 'Пример за getPagesPublic'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следващите примери на код все още са в бета. При проблеми, моля докладвайте чрез http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let cursor = "cursor_example" // String | Непрозрачен курсор за странициране, врънат като `nextCursor` от предишна заявка. Свързан е със същия `sortBy`. (незадължително)
let limit = 987 // Int | 1..200, по подразбиране 50 (незадължително)
let q = "q_example" // String | Незадължителен регистронезависим филтър по префикс на заглавие. (незадължително)
let sortBy = PagesSortBy() // PagesSortBy | Подредба. `updatedAt` (по подразбиране, най-новите първи), `commentCount` (най-много коментари първи), или `title` (азбучно). (незадължително)
let hasComments = true // Bool | Ако е true, връща само страници с поне един коментар. (незадължително)

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