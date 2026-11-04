Prevodi date komentare na jezik vidžeta. Dostupno samo kada odgovarajuće pravilo prilagođavanja vidžeta ima Comment Translation postavljeno na Automatic ili Manual.

## Parametri

| Ime | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Da |  |
| urlId | string | query | Da |  |
| sso | string | query | Ne |  |
| locale | string | query | Ne |  |

## Odgovor

Vraća: [`CommentTranslationsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CommentTranslationsResponse.swift)

## Primer

[inline-code-attrs-start title = 'translateComments Primer'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 
let commentTranslationsRequest = CommentTranslationsRequest(commentIds: ["commentIds_example"]) // CommentTranslationsRequest | 
let sso = "sso_example" // String |  (optional)
let locale = "locale_example" // String |  (optional)

PublicAPI.translateComments(tenantId: tenantId, urlId: urlId, commentTranslationsRequest: commentTranslationsRequest, options: PublicAPI.TranslateCommentsOptions(sso: sso, locale: locale)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]