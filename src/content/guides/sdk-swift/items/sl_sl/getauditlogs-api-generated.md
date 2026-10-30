## Parametri

| Ime | Tip | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes | Najemnik, za katerega se berejo revizijski dnevniki. |
| limit | integer | query | No | Koliko dnevnikov naj se vrne. Največ 10000, privzeto 1000. |
| skip | integer | query | No | Koliko dnevnikov naj se preskoči, za straničenje. |
| order | string | query | No | Smer razvrščanja po času dogodka. Privzeto DESC (najprej najnovejše). |
| after | integer | query | No |  |
| before | integer | query | No |  |
| username | string | query | No | Vrni le dogodke, ki jih je izvedel to uporabniško ime. |
| ip | string | query | No | Vrni le dogodke z tega IP naslova. |
| crudType | string | query | No | Vrni le dogodke tega tipa: c, r, u, d ali login. |
| resourceName | string | query | No | Vrni le dogodke za ta vir, npr. User ali Moderator. |
| targetId | string | query | No | Vrni le dogodke, katerih prizadeti objekt ima ta ID. |
| target | string | query | No | Neobčutljivo na velikost črk delno ujemanje v oznaki prizadetega objekta, npr. uporabniško ime ali e-pošto. |
| includeManagedTenants | boolean | query | No | Vrni tudi dogodke najemnikov, ki jih ta najemnik upravlja. |

## Odgovor

Vrne: [`GetAuditLogsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetAuditLogsResponse.swift)

## Primer

[inline-code-attrs-start title = 'Primer getAuditLogs'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Naslednji vzorci kode so še v beta fazi. Za morebitne težave prosimo, da jih prijavite na http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | Najemnik, za katerega se berejo revizijski dnevniki.
let limit = 987 // Int | Koliko dnevnikov naj se vrne. Največ 10000, privzeto 1000. (neobvezno)
let skip = 987 // Int | Koliko dnevnikov naj se preskoči, za straničenje. (neobvezno)
let order = SORT_DIR() // SORTDIR | Smer razvrščanja po času dogodka. Privzeto DESC (najprej najnovejše). (neobvezno)
let after = 987 // Int64 |  (optional)
let before = 987 // Int64 |  (optional)
let username = "username_example" // String | Vrni le dogodke, ki jih je izvedel to uporabniško ime. (neobvezno)
let ip = "ip_example" // String | Vrni le dogodke z tega IP naslova. (neobvezno)
let crudType = "crudType_example" // String | Vrni le dogodke tega tipa: c, r, u, d ali login. (neobvezno)
let resourceName = "resourceName_example" // String | Vrni le dogodke za ta vir, npr. User ali Moderator. (neobvezno)
let targetId = "targetId_example" // String | Vrni le dogodke, katerih prizadeti objekt ima ta ID. (neobvezno)
let target = "target_example" // String | Neobčutljivo na velikost črk delno ujemanje v oznaki prizadetega objekta, npr. uporabniško ime ali e-pošto. (neobvezno)
let includeManagedTenants = true // Bool | Vrni tudi dogodke najemnikov, ki jih ta najemnik upravlja. (neobvezno)

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