---
Батцх информације о корисницима за тенанта. За датих userIds, враћа приказну информацију из User / SSOUser.
Користи га коментар видгет да обогати кориснике који су управо појавили путем presence догађаја.
Нема контекста странице: приватност се примењује униформно (приватни профили су прикривени).

## Параметри

| Име | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| ids | string | query | Yes | userIds раздвојени зарезом. |

## Одговор

Враћа: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersInfoResponse.swift)

## Пример

[inline-code-attrs-start title = 'getUsersInfo Пример'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следећи примери кода су још у бета фази. За било који проблем, пријавите га преко http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let ids = "ids_example" // String | userIds раздвојени зарезом.

PublicAPI.getUsersInfo(tenantId: tenantId, ids: ids) { (response, error) in
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