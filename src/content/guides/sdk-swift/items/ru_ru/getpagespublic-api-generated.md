Получение списка страниц для тенанта. Используется десктоп-клиентом FChat для заполнения списка комнат.
Требуется, чтобы в результирующей пользовательской конфигурации каждой страницы свойство `enableFChat` было true.
Страницы, требующие SSO, фильтруются с учётом доступа групп запрашивающего пользователя.

## Параметры

| Имя | Тип | Location | Обязательный | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| cursor | string | query | Нет | Непрозрачный курсор пагинации, возвращаемый как `nextCursor` в предыдущем запросе. Привязан к тому же `sortBy`. |
| limit | integer | query | Нет | 1..200, по умолчанию 50 |
| q | string | query | Нет | Необязательный регистронезависимый фильтр по префиксу заголовка. |
| sortBy | string | query | Нет | Порядок сортировки. `updatedAt` (по умолчанию, сначала самые новые), `commentCount` (сначала страницы с наибольшим количеством комментариев) или `title` (по алфавиту). |
| hasComments | boolean | query | Нет | Если true, возвращать только страницы с хотя бы одним комментарием. |

## Ответ

Возвращает: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPublicPagesResponse.swift)

## Пример

[inline-code-attrs-start title = 'Пример getPagesPublic'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следующие примеры кода всё ещё в бета-версии. По любым проблемам, пожалуйста, сообщите через http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let cursor = "cursor_example" // String | Непрозрачный курсор пагинации, возвращаемый как `nextCursor` в предыдущем запросе. Привязан к тому же `sortBy`. (необязательно)
let limit = 987 // Int | 1..200, по умолчанию 50 (необязательно)
let q = "q_example" // String | Необязательный регистронезависимый фильтр по префиксу заголовка. (необязательно)
let sortBy = PagesSortBy() // PagesSortBy | Порядок сортировки. `updatedAt` (по умолчанию, сначала самые новые), `commentCount` (сначала страницы с наибольшим количеством комментариев) или `title` (по алфавиту). (необязательно)
let hasComments = true // Bool | Если true, возвращать только страницы с хотя бы одним комментарием. (необязательно)

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

---