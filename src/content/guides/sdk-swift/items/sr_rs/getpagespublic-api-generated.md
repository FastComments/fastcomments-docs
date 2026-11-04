Наводи странице за закупца (tenant). Користи FChat десктоп клијент да попуни листу својих соба.
Захтева да `enableFChat` буде true у resolved custom config-у за сваку страницу.
Странице које захтевају SSO се филтрирају према приступу групи тражеоца.

## Параметри

| Назив | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| cursor | string | query | Не | Опацке пагинациони курсор враћен као `nextCursor` из претходног захтева. Везан за исти `sortBy`. |
| limit | integer | query | Не | 1..200, подразумевано 50 |
| q | string | query | Не | Опционо филтрирање по префиксу наслова, без разликовања великих и малих слова. |
| sortBy | string | query | Не | Редослед сортирања. `updatedAt` (подразумевано, прво најновије), `commentCount` (прво са највише коментара) или `title` (абецедно). |
| hasComments | boolean | query | Не | Ако је true, враћа само странице са бар једним коментаром. |

## Одговор

Враћа: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPublicPagesResponse.swift)

## Пример

[inline-code-attrs-start title = 'getPagesPublic пример'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следећи примери кода су још увек у бета верзији. За било какав проблем, пријавите преко http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let cursor = "cursor_example" // String | Опацке пагинациони курсор враћен као `nextCursor` из претходног захтева. Везан за исти `sortBy`. (optional)
let limit = 987 // Int | 1..200, подразумевано 50 (optional)
let q = "q_example" // String | Опционо филтрирање по префиксу наслова, без разликовања великих и малих слова. (optional)
let sortBy = PagesSortBy() // PagesSortBy | Редослед сортирања. `updatedAt` (подразумевано, прво најновије), `commentCount` (прво са највише коментара) или `title` (абецедно). (optional)
let hasComments = true // Bool | Ако је true, враћа само странице са бар једним коментаром. (optional)

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