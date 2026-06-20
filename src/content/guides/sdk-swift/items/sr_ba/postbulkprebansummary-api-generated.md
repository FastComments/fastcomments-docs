## Параметри

| Име | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| includeByUserIdAndEmail | boolean | query | Не |  |
| includeByIP | boolean | query | Не |  |
| includeByEmailDomain | boolean | query | Не |  |
| sso | string | query | Не |  |

## Одговор

Враћа: [`BulkPreBanSummary`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/BulkPreBanSummary.swift)

## Пример

[inline-code-attrs-start title = 'postBulkPreBanSummary Пример'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следећи примјери кода су још увијек у бети. За проблеме, пријавите их на http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let bulkPreBanParams = BulkPreBanParams(commentIds: ["commentIds_example"]) // BulkPreBanParams | 
let includeByUserIdAndEmail = true // Bool |  (опционо)
let includeByIP = true // Bool |  (опционо)
let includeByEmailDomain = true // Bool |  (опционо)
let sso = "sso_example" // String |  (опционо)

ModerationAPI.postBulkPreBanSummary(bulkPreBanParams: bulkPreBanParams, includeByUserIdAndEmail: includeByUserIdAndEmail, includeByIP: includeByIP, includeByEmailDomain: includeByEmailDomain, sso: sso) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]