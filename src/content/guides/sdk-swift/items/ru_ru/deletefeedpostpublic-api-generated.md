## Параметры

| Имя | Тип | Расположение | Обязательно | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| postId | string | path | Да |  |
| broadcastId | string | query | Нет |  |
| sso | string | query | Нет |  |

## Ответ

Возвращает: [`DeleteFeedPostPublicResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/DeleteFeedPostPublicResponse.swift)

## Пример

[inline-code-attrs-start title = 'Пример deleteFeedPostPublic'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Приведённые ниже примеры кода всё ещё находятся в бета‑версии. Сообщайте о любых проблемах через http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let postId = "postId_example" // String | 
let broadcastId = "broadcastId_example" // String |  (необязательно)
let sso = "sso_example" // String |  (необязательно)

PublicAPI.deleteFeedPostPublic(tenantId: tenantId, postId: postId, broadcastId: broadcastId, sso: sso) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]