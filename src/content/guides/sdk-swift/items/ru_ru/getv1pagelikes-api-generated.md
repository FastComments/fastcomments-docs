## Параметры

| Имя | Тип | Расположение | Обязательно | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| urlId | string | query | Да |  |
| sso | string | query | Нет |  |
| includeUserInfo | string | query | Нет | Когда установлен, возвращает пользователей, которым понравилась страница (аватары и имена пользователей). Только "public" принимается, что исключает приватные профили. |

## Ответ

Возвращает: [`GetV1PageLikes`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetV1PageLikes.swift)

## Пример

[inline-code-attrs-start title = 'Пример getV1PageLikes'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следующие примеры кода находятся в бета-версии. При возникновении проблем, пожалуйста, сообщайте по адресу http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 
let sso = "sso_example" // String |  (необязательно)
let includeUserInfo = PageReactIncludeUserInfo() // PageReactIncludeUserInfo | Когда установлен, возвращает пользователей, которым понравилась страница (аватары и имена пользователей). Только \"public\" принимается, что исключает приватные профили. (необязательно)

PublicAPI.getV1PageLikes(tenantId: tenantId, urlId: urlId, options: PublicAPI.GetV1PageLikesOptions(sso: sso, includeUserInfo: includeUserInfo)) { (response, error) in
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