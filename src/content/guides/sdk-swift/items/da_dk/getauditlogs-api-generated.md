## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes | Lejeren, som revisionslogfiler skal læses for. |
| limit | integer | query | No | Hvor mange logfiler der skal returneres. Max 10000, standard er 1000. |
| skip | integer | query | No | Hvor mange logfiler der skal springes over, for sideinddeling. |
| order | string | query | No | Sorteringsretning på hændelsestiden. Standard er DESC (nyeste først). |
| after | integer | query | No |  |
| before | integer | query | No |  |
| username | string | query | No | Returner kun hændelser udført af dette brugernavn. |
| ip | string | query | No | Returner kun hændelser fra denne IP-adresse. |
| crudType | string | query | No | Returner kun hændelser af denne type: c, r, u, d eller login. |
| resourceName | string | query | No | Returner kun hændelser for denne ressource, f.eks. Bruger eller Moderator. |
| targetId | string | query | No | Returner kun hændelser, hvor det berørte objekt har dette id. |
| target | string | query | No | Case‑insensitiv delstrengssøgning på det berørte objekts label, f.eks. et brugernavn eller en e‑mail. |
| includeManagedTenants | boolean | query | No | Returner også hændelser fra lejere, som denne lejer administrerer. |

## Svar

Returnerer: [`GetAuditLogsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetAuditLogsResponse.swift)

## Eksempel

[inline-code-attrs-start title = 'getAuditLogs Eksempel'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Følgende kodeeksempler er stadig i beta. For eventuelle problemer, rapporter venligst via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | Lejeren, som revisionslogfiler skal læses for.
let limit = 987 // Int | Hvor mange logfiler der skal returneres. Max 10000, standard er 1000. (valgfri)
let skip = 987 // Int | Hvor mange logfiler der skal springes over, for sideinddeling. (valgfri)
let order = SORT_DIR() // SORTDIR | Sorteringsretning på hændelsestiden. Standard er DESC (nyeste først). (valgfri)
let after = 987 // Int64 |  (optional)
let before = 987 // Int64 |  (optional)
let username = "username_example" // String | Returner kun hændelser udført af dette brugernavn. (valgfri)
let ip = "ip_example" // String | Returner kun hændelser fra denne IP-adresse. (valgfri)
let crudType = "crudType_example" // String | Returner kun hændelser af denne type: c, r, u, d eller login. (valgfri)
let resourceName = "resourceName_example" // String | Returner kun hændelser for denne ressource, f.eks. Bruger eller Moderator. (valgfri)
let targetId = "targetId_example" // String | Returner kun hændelser, hvor det berørte objekt har dette id. (valgfri)
let target = "target_example" // String | Case‑insensitiv delstrengssøgning på det berørte objekts label, f.eks. et brugernavn eller en e‑mail. (valgfri)
let includeManagedTenants = true // Bool | Returner også hændelser fra lejere, som denne lejer administrerer. (valgfri)

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