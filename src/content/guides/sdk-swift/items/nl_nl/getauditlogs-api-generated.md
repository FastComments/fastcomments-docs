## Parameters

| Naam | Type | Locatie | Verplicht | Beschrijving |
|------|------|----------|-----------|--------------|
| tenantId | string | query | Ja | De tenant waarvoor auditlogboeken moeten worden gelezen. |
| limit | integer | query | Nee | Hoeveel logboeken moeten worden geretourneerd. Max 10000, standaard 1000. |
| skip | integer | query | Nee | Hoeveel logboeken moeten worden overgeslagen, voor paginering. |
| order | string | query | Nee | Sorteerrichting op de gebeurtenistijd. Standaard DESC (nieuwste eerst). |
| after | integer | query | Nee |  |
| before | integer | query | Nee |  |
| username | string | query | Nee | Alleen gebeurtenissen retourneren die door deze gebruikersnaam zijn uitgevoerd. |
| ip | string | query | Nee | Alleen gebeurtenissen retourneren van dit IP-adres. |
| crudType | string | query | Nee | Alleen gebeurtenissen van dit type retourneren: c, r, u, d, of login. |
| resourceName | string | query | Nee | Alleen gebeurtenissen voor deze bron retourneren, bijv. User of Moderator. |
| targetId | string | query | Nee | Alleen gebeurtenissen waarvan het getroffen object dit id heeft retourneren. |
| target | string | query | Nee | Hoofdletterongevoelige substringmatch op het label van het getroffen object, bijv. een gebruikersnaam of e‑mail. |
| includeManagedTenants | boolean | query | Nee | Ook gebeurtenissen retourneren van tenants die deze tenant beheert. |

## Response

Retourneert: [`GetAuditLogsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetAuditLogsResponse.swift)

## Voorbeeld

[inline-code-attrs-start title = 'getAuditLogs Voorbeeld'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// De volgende codevoorbeelden zijn nog in bèta. Meld eventuele problemen via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | De tenant waarvoor auditlogboeken moeten worden gelezen.
let limit = 987 // Int | Hoeveel logboeken moeten worden geretourneerd. Max 10000, standaard 1000. (optioneel)
let skip = 987 // Int | Hoeveel logboeken moeten worden overgeslagen, voor paginering. (optioneel)
let order = SORT_DIR() // SORTDIR | Sorteerrichting op de gebeurtenistijd. Standaard DESC (nieuwste eerst). (optioneel)
let after = 987 // Int64 |  (optioneel)
let before = 987 // Int64 |  (optioneel)
let username = "username_example" // String | Alleen gebeurtenissen retourneren die door deze gebruikersnaam zijn uitgevoerd. (optioneel)
let ip = "ip_example" // String | Alleen gebeurtenissen retourneren van dit IP-adres. (optioneel)
let crudType = "crudType_example" // String | Alleen gebeurtenissen van dit type retourneren: c, r, u, d, of login. (optioneel)
let resourceName = "resourceName_example" // String | Alleen gebeurtenissen voor deze bron retourneren, bijv. User of Moderator. (optioneel)
let targetId = "targetId_example" // String | Alleen gebeurtenissen waarvan het getroffen object dit id heeft retourneren. (optioneel)
let target = "target_example" // String | Hoofdletterongevoelige substringmatch op het label van het getroffen object, bijv. een gebruikersnaam of e‑mail. (optioneel)
let includeManagedTenants = true // Bool | Ook gebeurtenissen retourneren van tenants die deze tenant beheert. (optioneel)

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

---