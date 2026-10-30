## Parametri

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|-----------|--------------|-------------|
| tenantId | string | path | Sì |  |
| urlId | string | query | Sì |  |
| sso | string | query | No |  |

## Risposta

Restituisce: [`CreateV1PageReact`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CreateV1PageReact.swift)

## Esempio

[inline-code-attrs-start title = 'Esempio deleteV1PageReact'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// I seguenti esempi di codice sono ancora beta. Per qualsiasi problema, si prega di segnalare tramite http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 
let sso = "sso_example" // String |  (opzionale)

PublicAPI.deleteV1PageReact(tenantId: tenantId, urlId: urlId, sso: sso) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]