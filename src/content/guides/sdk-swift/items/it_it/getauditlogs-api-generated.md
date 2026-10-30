## Parameters

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|-----------|--------------|-------------|
| tenantId | string | query | Sì | Il tenant per cui leggere i log di audit. |
| limit | integer | query | No | Quanti log restituire. Max 10000, valore predefinito 1000. |
| skip | integer | query | No | Quanti log saltare, per paginazione. |
| order | string | query | No | Direzione di ordinamento sul tempo dell'evento. Predefinito DESC (più recenti per primi). |
| after | integer | query | No |  |
| before | integer | query | No |  |
| username | string | query | No | Restituisce solo gli eventi eseguiti da questo nome utente. |
| ip | string | query | No | Restituisce solo gli eventi da questo indirizzo IP. |
| crudType | string | query | No | Restituisce solo gli eventi di questo tipo: c, r, u, d, o login. |
| resourceName | string | query | No | Restituisce solo gli eventi per questa risorsa, ad es. User o Moderator. |
| targetId | string | query | No | Restituisce solo gli eventi il cui oggetto interessato ha questo id. |
| target | string | query | No | Corrispondenza di sottostringa non sensibile al maiuscolo/minuscolo sull'etichetta dell'oggetto interessato, ad es. un nome utente o email. |
| includeManagedTenants | boolean | query | No | Restituisce anche gli eventi dai tenant gestiti da questo tenant. |

## Response

Restituisce: [`GetAuditLogsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetAuditLogsResponse.swift)

## Example

[inline-code-attrs-start title = 'Esempio getAuditLogs'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// I seguenti esempi di codice sono ancora in beta. Per qualsiasi problema, si prega di segnalare via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | Il tenant per cui leggere i log di audit.
let limit = 987 // Int | Quanti log restituire. Max 10000, valore predefinito 1000. (opzionale)
let skip = 987 // Int | Quanti log saltare, per paginazione. (opzionale)
let order = SORT_DIR() // SORTDIR | Direzione di ordinamento sul tempo dell'evento. Predefinito DESC (più recenti per primi). (opzionale)
let after = 987 // Int64 |  (opzionale)
let before = 987 // Int64 |  (opzionale)
let username = "username_example" // String | Restituisce solo gli eventi eseguiti da questo nome utente. (opzionale)
let ip = "ip_example" // String | Restituisce solo gli eventi da questo indirizzo IP. (opzionale)
let crudType = "crudType_example" // String | Restituisce solo gli eventi di questo tipo: c, r, u, d, o login. (opzionale)
let resourceName = "resourceName_example" // String | Restituisce solo gli eventi per questa risorsa, ad es. User o Moderator. (opzionale)
let targetId = "targetId_example" // String | Restituisce solo gli eventi il cui oggetto interessato ha questo id. (opzionale)
let target = "target_example" // String | Corrispondenza di sottostringa non sensibile al maiuscolo/minuscolo sull'etichetta dell'oggetto interessato, ad es. un nome utente o email. (opzionale)
let includeManagedTenants = true // Bool | Restituisce anche gli eventi dai tenant gestiti da questo tenant. (opzionale)

DefaultAPI.getAuditLogs(tenantId: tenantId, options: DefaultAPI.GetAuditLogsOptions(limit: limit, skip: skip, order: order, after: after, before: before, username: username, ip: ip, crudType: crudType, resourceName: resourceName, targetId: targetId, target: target, includeManagedTenants: includeManagedTenants)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]