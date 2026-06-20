---
Spettatori attualmente online di una pagina: persone la cui sessione websocket è sottoscritta alla pagina in questo momento.
Restituisce anonCount + totalCount (iscritti a livello di stanza, inclusi i visualizzatori anonimi che non elenchiamo).

## Parametri

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|----------|----------|-------------|
| tenantId | string | path | Sì |  |
| urlId | string | query | Sì | Identificatore dell'URL della pagina (ripulito lato server). |
| afterName | string | query | No | Cursore: passare nextAfterName dalla risposta precedente. |
| afterUserId | string | query | No | Cursore di spareggio: passare nextAfterUserId dalla risposta precedente. Richiesto quando afterName è impostato in modo che i pareggi per nome non facciano scartare voci. |

## Risposta

Restituisce: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersOnlineResponse.swift)

## Esempio

[inline-code-attrs-start title = 'Esempio di getOnlineUsers'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// I seguenti esempi di codice sono ancora in beta. Per qualsiasi problema, si prega di segnalare tramite http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | Identificatore dell'URL della pagina (ripulito lato server).
let afterName = "afterName_example" // String | Cursore: passare nextAfterName dalla risposta precedente. (opzionale)
let afterUserId = "afterUserId_example" // String | Cursore di spareggio: passare nextAfterUserId dalla risposta precedente. Richiesto quando afterName è impostato in modo che i pareggi per nome non facciano scartare voci. (opzionale)

PublicAPI.getOnlineUsers(tenantId: tenantId, urlId: urlId, afterName: afterName, afterUserId: afterUserId) { (response, error) in
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