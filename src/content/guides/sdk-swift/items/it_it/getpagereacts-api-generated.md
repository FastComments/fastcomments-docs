---
Conteggi di reazioni personalizzate per una pagina, opzionalmente con gli utenti che hanno reagito.

## Parametri

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|------------|--------------|-------------|
| tenantId | string | query | Sì |  |
| urlId | string | query | Sì |  |
| includeUserInfo | string | query | No | "\"public\" omette gli utenti con profili privati, \"include-private\" li include." |

## Risposta

Restituisce: [`GetPageReactsAPIResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPageReactsAPIResponse.swift)

## Esempio

[inline-code-attrs-start title = 'Esempio getPageReacts'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// I seguenti esempi di codice sono ancora in beta. Per qualsiasi problema, si prega di segnalare via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 
let includeUserInfo = PageReactIncludeUserInfo() // PageReactIncludeUserInfo | \"public\" omette gli utenti con profili privati, \"include-private\" li include. (optional)

DefaultAPI.getPageReacts(tenantId: tenantId, urlId: urlId, includeUserInfo: includeUserInfo) { (response, error) in
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