## Parametri

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|----------|--------------|-------------|
| tenantId | string | query | Sì |  |
| commentId | string | path | Sì |  |

## Risposta

Restituisce: [`GetPollResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPollResponse.swift)

## Esempio

[inline-code-attrs-start title = 'Esempio getPoll'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// I seguenti esempi di codice sono ancora in beta. Per qualsiasi problema, si prega di segnalare via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String | 

DefaultAPI.getPoll(tenantId: tenantId, commentId: commentId) { (response, error) in
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