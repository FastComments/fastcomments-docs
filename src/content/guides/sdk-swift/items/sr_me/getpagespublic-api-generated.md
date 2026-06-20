Набраја странице за тенанта. Користи га FChat десктоп клијент да попуни своју листу соба. Захтијева да `enableFChat` буде true у разрешеном прилагођеном конфигу за сваку страницу. Странице које захтијевају SSO филтрују се према приступним групама корисника који прави захтјев.

## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Непрозирни курсор пагинације враћен као `nextCursor` из претходног захтјева. Повезан са истим `sortBy`. |
| limit | integer | query | No | 1..200, подразумевано 50 |
| q | string | query | No | Опциони префикс филтер по наслову који не прави разлику између великих и малих слова. |
| sortBy | string | query | No | Редослијед сортирања. `updatedAt` (подразумевано, најновије прво), `commentCount` (највише коментара прво) или `title` (абецедни редослијед). |
| hasComments | boolean | query | No | Ако је true, враћају се само странице са бар једним коментаром. |

## Response

Враћа: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPublicPagesResponse.swift)

## Пример

[inline-code-attrs-start title = 'getPagesPublic Пример'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следећи примјери кода су још у бета фази. За било који проблем, пријавите га преко http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let cursor = "cursor_example" // String | Непрозирни курсор пагинације враћен као `nextCursor` из претходног захтјева. Повезан са истим `sortBy`. (опционо)
let limit = 987 // Int | 1..200, подразумевано 50 (опционо)
let q = "q_example" // String | Опциони префикс филтер по наслову који не прави разлику између великих и малих слова. (опционо)
let sortBy = PagesSortBy() // PagesSortBy | Редослијед сортирања. `updatedAt` (подразумевано, најновије прво), `commentCount` (највише коментара прво), или `title` (абецедни редослијед). (опционо)
let hasComments = true // Bool | Ако је true, враћају се само странице са најмање једним коментаром. (опционо)

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