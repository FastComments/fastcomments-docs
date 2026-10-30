## Parameters

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|------|
| tenantId | string | query | Yes | Najam za čitanje revizijskih zapisa. |
| limit | integer | query | No | Koliko zapisa vratiti. Maksimalno 10000, zadano 1000. |
| skip | integer | query | No | Koliko zapisa preskočiti, za paginaciju. |
| order | string | query | No | Smjer sortiranja po vremenu događaja. Zadano DESC (najnoviji prvi). |
| after | integer | query | No |  |
| before | integer | query | No |  |
| username | string | query | No | Vraća samo događaje izvršene od strane ovog korisničkog imena. |
| ip | string | query | No | Vraća samo događaje s ove IP adrese. |
| crudType | string | query | No | Vraća samo događaje ovog tipa: c, r, u, d ili login. |
| resourceName | string | query | No | Vraća samo događaje za ovaj resurs, npr. User ili Moderator. |
| targetId | string | query | No | Vraća samo događaje čiji je zahvaćeni objekt s ovim ID-om. |
| target | string | query | No | Podudaranje podstringa neosjetljivo na veličinu slova na oznaku zahvaćenog objekta, npr. korisničko ime ili e‑mail. |
| includeManagedTenants | boolean | query | No | Također vraća događaje od najma koje ovaj najam upravlja. |

## Response

Vraća: [`GetAuditLogsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetAuditLogsResponse.swift)

## Example

[inline-code-attrs-start title = 'Primjer getAuditLogs'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Sljedeći primjeri koda su još u beta fazi. Za bilo koji problem, molimo prijavite ga putem http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | Najam za čitanje revizijskih zapisa.
let limit = 987 // Int | Koliko zapisa vratiti. Maksimalno 10000, zadano 1000. (opcionalno)
let skip = 987 // Int | Koliko zapisa preskočiti, za paginaciju. (opcionalno)
let order = SORT_DIR() // SORTDIR | Smjer sortiranja po vremenu događaja. Zadano DESC (najnoviji prvi). (opcionalno)
let after = 987 // Int64 |  (opcionalno)
let before = 987 // Int64 |  (opcionalno)
let username = "username_example" // String | Vraća samo događaje izvršene od strane ovog korisničkog imena. (opcionalno)
let ip = "ip_example" // String | Vraća samo događaje s ove IP adrese. (opcionalno)
let crudType = "crudType_example" // String | Vraća samo događaje ovog tipa: c, r, u, d ili login. (opcionalno)
let resourceName = "resourceName_example" // String | Vraća samo događaje za ovaj resurs, npr. User ili Moderator. (opcionalno)
let targetId = "targetId_example" // String | Vraća samo događaje čiji je zahvaćeni objekt s ovim ID-om. (opcionalno)
let target = "target_example" // String | Podudaranje podstringa neosjetljivo na veličinu slova na oznaku zahvaćenog objekta, npr. korisničko ime ili e‑mail. (opcionalno)
let includeManagedTenants = true // Bool | Također vraća događaje od najma koje ovaj najam upravlja. (opcionalno)

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