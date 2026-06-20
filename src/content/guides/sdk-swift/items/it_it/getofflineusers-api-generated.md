---
Commentatori passati sulla pagina che NON sono attualmente online. Ordinati per displayName.
Usalo dopo aver esaurito /users/online per visualizzare una sezione "Membri".
Paginazione con cursore su commenterName: il server scorre l'indice parziale {tenantId, urlId, commenterName} da afterName in avanti usando $gt, senza costo $skip.

## Parameters

| Nome | Tipo | Posizione | Richiesto | Descrizione |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Identificatore URL della pagina (ripulito lato server). |
| afterName | string | query | No | Cursore: passare nextAfterName dalla risposta precedente. |
| afterUserId | string | query | No | Cursore per rompere i pareggi: passare nextAfterUserId dalla risposta precedente. Richiesto quando afterName è impostato in modo che i pareggi di nome non facciano perdere voci. |

## Response

Restituisce: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersOfflineResponse.swift)

## Example

[inline-code-attrs-start title = 'Esempio di getOfflineUsers'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// I seguenti esempi di codice sono ancora in beta. Per qualsiasi problema, segnalarlo tramite http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | Identificatore URL della pagina (ripulito lato server).
let afterName = "afterName_example" // String | Cursore: passare nextAfterName dalla risposta precedente. (opzionale)
let afterUserId = "afterUserId_example" // String | Cursore per rompere i pareggi: passare nextAfterUserId dalla risposta precedente. Richiesto quando afterName è impostato in modo che i pareggi di nome non facciano perdere voci. (opzionale)

PublicAPI.getOfflineUsers(tenantId: tenantId, urlId: urlId, afterName: afterName, afterUserId: afterUserId) { (response, error) in
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