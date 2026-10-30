## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes | Le locataire dont il faut lire les journaux d'audit. |
| limit | integer | query | No | Nombre de journaux à renvoyer. Max 10000, par défaut 1000. |
| skip | integer | query | No | Nombre de journaux à ignorer, pour la pagination. |
| order | string | query | No | Direction de tri sur le temps de l'événement. Par défaut DESC (du plus récent au plus ancien). |
| after | integer | query | No |  |
| before | integer | query | No |  |
| username | string | query | No | Ne renvoie que les événements effectués par cet utilisateur. |
| ip | string | query | No | Ne renvoie que les événements provenant de cette adresse IP. |
| crudType | string | query | No | Ne renvoie que les événements de ce type : c, r, u, d, ou login. |
| resourceName | string | query | No | Ne renvoie que les événements pour cette ressource, par ex. User ou Moderator. |
| targetId | string | query | No | Ne renvoie que les événements dont l'objet affecté possède cet identifiant. |
| target | string | query | No | Correspondance insensible à la casse d'une sous-chaîne sur le libellé de l'objet affecté, par ex. un nom d'utilisateur ou un e‑mail. |
| includeManagedTenants | boolean | query | No | Renvoie également les événements des locataires gérés par ce locataire. |

## Réponse

Renvoie : [`GetAuditLogsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetAuditLogsResponse.swift)

## Exemple

[inline-code-attrs-start title = 'Exemple getAuditLogs'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Les exemples de code suivants sont encore en version bêta. Pour tout problème, veuillez le signaler via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | Le locataire dont il faut lire les journaux d'audit.
let limit = 987 // Int | Nombre de journaux à renvoyer. Max 10000, par défaut 1000. (optionnel)
let skip = 987 // Int | Nombre de journaux à ignorer, pour la pagination. (optionnel)
let order = SORT_DIR() // SORTDIR | Direction de tri sur le temps de l'événement. Par défaut DESC (du plus récent au plus ancien). (optionnel)
let after = 987 // Int64 |  (optionnel)
let before = 987 // Int64 |  (optionnel)
let username = "username_example" // String | Ne renvoie que les événements effectués par cet utilisateur. (optionnel)
let ip = "ip_example" // String | Ne renvoie que les événements provenant de cette adresse IP. (optionnel)
let crudType = "crudType_example" // String | Ne renvoie que les événements de ce type : c, r, u, d, ou login. (optionnel)
let resourceName = "resourceName_example" // String | Ne renvoie que les événements pour cette ressource, par ex. User ou Moderator. (optionnel)
let targetId = "targetId_example" // String | Ne renvoie que les événements dont l'objet affecté possède cet identifiant. (optionnel)
let target = "target_example" // String | Correspondance insensible à la casse d'une sous-chaîne sur le libellé de l'objet affecté, par ex. un nom d'utilisateur ou un e‑mail. (optionnel)
let includeManagedTenants = true // Bool | Renvoie également les événements des locataires gérés par ce locataire. (optionnel)

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