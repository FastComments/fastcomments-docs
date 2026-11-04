Выводит список страниц для тенанта. Используется десктоп-клиентом FChat для заполнения списка комнат.
Требует, чтобы `enableFChat` был равен true в resolved custom config для каждой страницы.
Страницы, требующие SSO, фильтруются с учётом группового доступа запрашивающего пользователя.

## Параметры

| Имя | Тип | Расположение | Обязательный | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| cursor | string | query | Нет | Непрозрачный курсор пагинации, возвращаемый как `nextCursor` из предыдущего запроса. Привязан к тому же `sortBy`. |
| limit | integer | query | Нет | 1..200, по умолчанию 50 |
| q | string | query | Нет | Необязательный фильтр по префиксу заголовка без учёта регистра. |
| sortBy | string | query | Нет | Порядок сортировки. `updatedAt` (по умолчанию, сначала новые), `commentCount` (сначала с наибольшим числом комментариев) или `title` (по алфавиту). |
| hasComments | boolean | query | Нет | Если true, возвращать только страницы, у которых есть хотя бы один комментарий. |

## Ответ

Возвращает: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPublicPagesResponse.swift)

## Пример

[inline-code-attrs-start title = 'Пример getPagesPublic'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let cursor = "cursor_example" // String | Непрозрачный курсор пагинации, возвращаемый как `nextCursor` из предыдущего запроса. Привязан к тому же `sortBy`. (optional)
let limit = 987 // Int | 1..200, по умолчанию 50 (optional)
let q = "q_example" // String | Необязательный фильтр по префиксу заголовка без учёта регистра. (optional)
let sortBy = PagesSortBy() // PagesSortBy | Порядок сортировки. `updatedAt` (по умолчанию, сначала новые), `commentCount` (сначала с наибольшим числом комментариев) или `title` (по алфавиту). (optional)
let hasComments = true // Bool | Если true, возвращать только страницы, у которых есть хотя бы один комментарий. (optional)

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