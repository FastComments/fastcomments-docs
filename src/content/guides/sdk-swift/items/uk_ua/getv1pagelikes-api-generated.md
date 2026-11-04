## Параметри

| Назва | Тип | Розташування | Обов'язковий | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | path | Так |  |
| urlId | string | query | Так |  |
| sso | string | query | Ні |  |
| includeUserInfo | string | query | Ні | Якщо встановлено, повертає користувачів, які вподобали сторінку (аватари та імена користувачів). Приймається лише "public", що виключає приватні профілі. |

## Відповідь

Повертає: [`GetV1PageLikes`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetV1PageLikes.swift)

## Приклад

[inline-code-attrs-start title = 'Приклад getV1PageLikes'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Наведені нижче приклади коду все ще є бета-версією. У разі виникнення проблем, будь ласка, повідомте на http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 
let sso = "sso_example" // String |  (optional)
let includeUserInfo = PageReactIncludeUserInfo() // PageReactIncludeUserInfo | Якщо встановлено, повертає користувачів, які вподобали сторінку (аватари та імена користувачів). Приймається лише \"public\", що виключає приватні профілі. (optional)

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