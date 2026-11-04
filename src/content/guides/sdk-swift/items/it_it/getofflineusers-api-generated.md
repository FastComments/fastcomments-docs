Commentatori precedenti sulla pagina che NON sono attualmente online. Ordinati per displayName.
Usare questo dopo aver esaurito /users/online per visualizzare una sezione "Members".
Paginazione con cursore su commenterName: il server percorre l'indice parziale {tenantId, urlId, commenterName}
da afterName in avanti tramite $gt, senza costo $skip.

## Parametri

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|----------|----------|-------------|
| tenantId | string | path | Sì |  |
| urlId | string | query | Sì | Identificatore dell'URL della pagina (pulito lato server). |
| afterName | string | query | No | Cursore: passare nextAfterName dalla risposta precedente. |
| afterUserId | string | query | No | Tiebreaker del cursore: passare nextAfterUserId dalla risposta precedente. Obbligatorio quando afterName è impostato, in modo che le voci con lo stesso nome non vengano perse. |

## Risposta

Restituisce: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersOfflineResponse.swift)

## Esempio

[inline-code-attrs-start title = 'Esempio di getOfflineUsers'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | Identificatore dell'URL della pagina (pulito lato server).
let afterName = "afterName_example" // String | Cursore: passare nextAfterName dalla risposta precedente. (optional)
let afterUserId = "afterUserId_example" // String | Tiebreaker del cursore: passare nextAfterUserId dalla risposta precedente. Obbligatorio quando afterName è impostato, in modo che le voci con lo stesso nome non vengano perse. (optional)

PublicAPI.getOfflineUsers(tenantId: tenantId, urlId: urlId, options: PublicAPI.GetOfflineUsersOptions(afterName: afterName, afterUserId: afterUserId)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]