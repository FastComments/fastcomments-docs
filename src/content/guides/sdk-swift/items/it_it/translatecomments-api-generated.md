Transla i commenti forniti nella lingua del widget. Disponibile solo quando la regola di personalizzazione del widget corrispondente ha la Traduzione dei Commenti impostata su Automatico o Manuale.

## Parametri

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|-----------|--------------|-------------|
| tenantId | string | path | Sì |  |
| urlId | string | query | Sì |  |
| sso | string | query | No |  |
| locale | string | query | No |  |

## Risposta

Restituisce: [`CommentTranslationsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CommentTranslationsResponse.swift)

## Esempio

[inline-code-attrs-start title = 'Esempio translateComments'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// I seguenti esempi di codice sono ancora in beta. Per qualsiasi problema, si prega di segnalare via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 
let commentTranslationsRequest = CommentTranslationsRequest(commentIds: ["commentIds_example"]) // CommentTranslationsRequest | 
let sso = "sso_example" // String |  (opzionale)
let locale = "locale_example" // String |  (opzionale)

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