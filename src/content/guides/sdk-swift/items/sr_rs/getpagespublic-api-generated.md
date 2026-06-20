Листа страница за тенанта. Користи се од стране FChat десктоп клијента за попуњавање листе соба.
Захтева да `enableFChat` буде true у резолвованој прилагођеној конфигурацији за сваку страницу.
Странице које захтевају SSO се филтрирају на основу приступа групе корисника који шаље захтев.

## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Нечитљив курсор пагинације враћен као `nextCursor` из претходног захтева. Повезан са истим `sortBy`. |
| limit | integer | query | No | 1..200, подразумевано 50 |
| q | string | query | No | Опционо филтрирање префикса наслова без обзира на регистар. |
| sortBy | string | query | No | Редослед сортирања. `updatedAt` (подразумевано, најновије прво), `commentCount` (највише коментара прво), или `title` (алфабетски). |
| hasComments | boolean | query | No | Ако је true, враћају се само странице које имају бар један коментар. |

## Одговор

Враћа: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPublicPagesResponse.swift)

## Пример

[inline-code-attrs-start title = 'Пример getPagesPublic'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следећи примери кода су још у бета фази. За било који проблем пријавите га преко http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let cursor = "cursor_example" // String | Опаки курсор пагинације враћен као `nextCursor` из претходног захтева. Повезан са истим `sortBy`. (опционо)
let limit = 987 // Int | 1..200, подразумевано 50 (опционо)
let q = "q_example" // String | Опционо филтрирање префикса наслова без обзира на регистар. (опционо)
let sortBy = PagesSortBy() // PagesSortBy | Редослед сортирања. `updatedAt` (подразумевано, најновије прво), `commentCount` (највише коментара прво), или `title` (алфабетски). (опционо)
let hasComments = true // Bool | Ако је true, враћају се само странице које имају бар један коментар. (опционо)

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