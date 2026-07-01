Списък на страниците за наемател. Използва се от десктоп клиента FChat за попълване на списъка с помещения.  
Изисква `enableFChat` да бъде истина в разрешената персонализирана конфигурация за всяка страница.  
Страници, които изискват SSO, се филтрират спрямо достъпа до групите на заявилия потребител.

## Параметри

| Име | Тип | Местоположение | Задължително | Описание |
|------|------|----------------|--------------|----------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Непрозрачен курсор за странициране, върнат като `nextCursor` от предишна заявка. Свързан със същия `sortBy`. |
| limit | integer | query | No | 1..200, default 50 |
| q | string | query | No | Опционален филтър за префикс на заглавието, нечувствителен към регистъра. |
| sortBy | string | query | No | Ред на сортиране. `updatedAt` (по подразбиране, най-новите първи), `commentCount` (най-много коментари първо), или `title` (по азбучен ред). |
| hasComments | boolean | query | No | Ако е истина, се връщат само страници с поне един коментар. |

## Отговор

Returns: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPublicPagesResponse.swift)

## Пример

[inline-code-attrs-start title = 'getPagesPublic Пример'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let cursor = "cursor_example" // String | Opaque pagination cursor returned as `nextCursor` from a prior request. Tied to the same `sortBy`. (optional)
let limit = 987 // Int | 1..200, default 50 (optional)
let q = "q_example" // String | Optional case-insensitive title prefix filter. (optional)
let sortBy = PagesSortBy() // PagesSortBy | Sort order. `updatedAt` (default, newest first), `commentCount` (most comments first), or `title` (alphabetical). (optional)
let hasComments = true // Bool | If true, only return pages with at least one comment. (optional)

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