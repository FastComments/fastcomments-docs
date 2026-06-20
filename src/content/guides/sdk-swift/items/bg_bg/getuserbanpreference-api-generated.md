## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| sso | string | query | Не |  |

## Отговор

Връща: [`APIModerateGetUserBanPreferencesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/APIModerateGetUserBanPreferencesResponse.swift)

## Пример

[inline-code-attrs-start title = 'Пример за getUserBanPreference'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следните примерни кодове все още са в бета версия. При проблем, моля докладвайте чрез http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let sso = "sso_example" // String | (по избор)

ModerationAPI.getUserBanPreference(sso: sso) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]