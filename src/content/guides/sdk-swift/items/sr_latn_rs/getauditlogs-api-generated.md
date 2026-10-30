## Parameters

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|------|
| tenantId | string | query | Da | Tenant za čitanje audit logova. |
| limit | integer | query | Ne | Koliko logova da se vrati. Maksimum 10000, podrazumevano 1000. |
| skip | integer | query | Ne | Koliko logova da se preskoči, za paginaciju. |
| order | string | query | Ne | Smer sortiranja po vremenu događaja. Podrazumevano DESC (najnoviji prvi). |
| after | integer | query | Ne |  |
| before | integer | query | Ne |  |
| username | string | query | Ne | Vraćaju se samo događaji izvršeni od strane ovog korisničkog imena. |
| ip | string | query | Ne | Vraćaju se samo događaji sa ove IP adrese. |
| crudType | string | query | Ne | Vraćaju se samo događaji ovog tipa: c, r, u, d, ili login. |
| resourceName | string | query | Ne | Vraćaju se samo događaji za ovaj resurs, npr. User ili Moderator. |
| targetId | string | query | Ne | Vraćaju se samo događaji čiji pogođeni objekat ima ovaj ID. |
| target | string | query | Ne | Podudaranje podstringa neosetljivo na veličinu slova na oznaci pogođenog objekta, npr. korisničko ime ili email. |
| includeManagedTenants | boolean | query | Ne | Takođe vraća događaje od tenant-a koje ovaj tenant upravlja. |

## Response

Vraća: [`GetAuditLogsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetAuditLogsResponse.swift)

## Example

[inline-code-attrs-start title = 'getAuditLogs Primer'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Sledeći primeri koda su i dalje beta. Za bilo koji problem, molimo prijavite na http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | Tenant za čitanje audit logova.
let limit = 987 // Int | Koliko logova da se vrati. Maksimum 10000, podrazumevano 1000. (optional)
let skip = 987 // Int | Koliko logova da se preskoči, za paginaciju. (optional)
let order = SORT_DIR() // SORTDIR | Smer sortiranja po vremenu događaja. Podrazumevano DESC (najnoviji prvi). (optional)
let after = 987 // Int64 |  (optional)
let before = 987 // Int64 |  (optional)
let username = "username_example" // String | Vraćaju se samo događaji izvršeni od strane ovog korisničkog imena. (optional)
let ip = "ip_example" // String | Vraćaju se samo događaji sa ove IP adrese. (optional)
let crudType = "crudType_example" // String | Vraćaju se samo događaji ovog tipa: c, r, u, d, ili login. (optional)
let resourceName = "resourceName_example" // String | Vraćaju se samo događaji za ovaj resurs, npr. User ili Moderator. (optional)
let targetId = "targetId_example" // String | Vraćaju se samo događaji čiji pogođeni objekat ima ovaj ID. (optional)
let target = "target_example" // String | Podudaranje podstringa neosetljivo na veličinu slova na oznaci pogođenog objekta, npr. korisničko ime ili email. (optional)
let includeManagedTenants = true // Bool | Takođe vraća događaje od tenant-a koje ovaj tenant upravlja. (optional)

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