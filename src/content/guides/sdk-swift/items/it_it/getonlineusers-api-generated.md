Attualmente online visualizzatori di una pagina: persone la cui sessione websocket è iscritta alla pagina in questo momento.  
Restituisce anonCount + totalCount (abbonati a livello di stanza, inclusi i visualizzatori anonimi che non elenchiamo).

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Identificatore URL della pagina (pulito lato server). |
| afterName | string | query | No | Cursore: passa nextAfterName dalla risposta precedente. |
| afterUserId | string | query | No | Cursore di scontro: passa nextAfterUserId dalla risposta precedente. Richiesto quando afterName è impostato affinché i pareggi di nome non elimino voci. |

## Response

Restituisce: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersOnlineResponse.swift)

## Example

[inline-code-attrs-start title = 'Esempio getOnlineUsers'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// I seguenti esempi di codice sono ancora beta. Per qualsiasi problema, si prega di segnalare via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | Identificatore URL della pagina (pulito lato server).
let afterName = "afterName_example" // String | Cursore: passa nextAfterName dalla risposta precedente. (opzionale)
let afterUserId = "afterUserId_example" // String | Cursore di scontro: passa nextAfterUserId dalla risposta precedente. Richiesto quando afterName è impostato affinché i pareggi di nome non elimino voci. (opzionale)

PublicAPI.getOnlineUsers(tenantId: tenantId, urlId: urlId, options: PublicAPI.GetOnlineUsersOptions(afterName: afterName, afterUserId: afterUserId)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]