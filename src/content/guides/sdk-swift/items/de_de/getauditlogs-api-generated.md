## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja | Der Mandant, für den die Audit-Logs gelesen werden sollen. |
| limit | integer | query | Nein | Wie viele Logs zurückgegeben werden sollen. Maximal 10000, Standardwert 1000. |
| skip | integer | query | Nein | Wie viele Logs übersprungen werden sollen, für die Seitennavigation. |
| order | string | query | Nein | Sortierrichtung nach der Ereigniszeit. Standard ist DESC (neueste zuerst). |
| after | integer | query | Nein |  |
| before | integer | query | Nein |  |
| username | string | query | Nein | Nur Ereignisse zurückgeben, die von diesem Benutzernamen durchgeführt wurden. |
| ip | string | query | Nein | Nur Ereignisse von dieser IP-Adresse zurückgeben. |
| crudType | string | query | Nein | Nur Ereignisse dieses Typs zurückgeben: c, r, u, d oder login. |
| resourceName | string | query | Nein | Nur Ereignisse für diese Ressource zurückgeben, z. B. User oder Moderator. |
| targetId | string | query | Nein | Nur Ereignisse zurückgeben, deren betroffener Gegenstand diese ID hat. |
| target | string | query | Nein | Fallunabhängige Teilstring‑Übereinstimmung im Label des betroffenen Objekts, z. B. ein Benutzername oder eine E‑Mail. |
| includeManagedTenants | boolean | query | Nein | Auch Ereignisse von Mandanten zurückgeben, die dieser Mandant verwaltet. |

## Antwort

Rückgabe: [`GetAuditLogsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetAuditLogsResponse.swift)

## Beispiel

[inline-code-attrs-start title = 'getAuditLogs Beispiel'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Die folgenden Codebeispiele sind noch in der Beta-Phase. Bei Problemen bitte melden über http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | Der Mandant, für den die Audit-Logs gelesen werden sollen.
let limit = 987 // Int | Wie viele Logs zurückgegeben werden sollen. Maximal 10000, Standardwert 1000. (optional)
let skip = 987 // Int | Wie viele Logs übersprungen werden sollen, für die Seitennavigation. (optional)
let order = SORT_DIR() // SORTDIR | Sortierrichtung nach der Ereigniszeit. Standard ist DESC (neueste zuerst). (optional)
let after = 987 // Int64 |  (optional)
let before = 987 // Int64 |  (optional)
let username = "username_example" // String | Nur Ereignisse zurückgeben, die von diesem Benutzernamen durchgeführt wurden. (optional)
let ip = "ip_example" // String | Nur Ereignisse von dieser IP-Adresse zurückgeben. (optional)
let crudType = "crudType_example" // String | Nur Ereignisse dieses Typs zurückgeben: c, r, u, d oder login. (optional)
let resourceName = "resourceName_example" // String | Nur Ereignisse für diese Ressource zurückgeben, z. B. User oder Moderator. (optional)
let targetId = "targetId_example" // String | Nur Ereignisse zurückgeben, deren betroffener Gegenstand diese ID hat. (optional)
let target = "target_example" // String | Fallunabhängige Teilstring‑Übereinstimmung im Label des betroffenen Objekts, z. B. ein Benutzername oder eine E‑Mail. (optional)
let includeManagedTenants = true // Bool | Auch Ereignisse von Mandanten zurückgeben, die dieser Mandant verwaltet. (optional)

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