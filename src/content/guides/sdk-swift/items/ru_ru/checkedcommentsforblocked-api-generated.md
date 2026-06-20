## Параметры

| Имя | Тип | Расположение | Обязательно | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| commentIds | string | query | Да | Список идентификаторов комментариев, разделённых запятыми. |
| sso | string | query | Нет |  |

## Ответ

Возвращает: [`CheckBlockedCommentsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CheckBlockedCommentsResponse.swift)

## Пример

[inline-code-attrs-start title = 'checkedCommentsForBlocked Пример'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следующие примеры кода все ещё в бета-версии. По возникшим проблемам, пожалуйста, сообщайте через http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentIds = "commentIds_example" // String | Список идентификаторов комментариев, разделённых запятыми.
let sso = "sso_example" // String |  (необязательно)

PublicAPI.checkedCommentsForBlocked(tenantId: tenantId, commentIds: commentIds, sso: sso) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]