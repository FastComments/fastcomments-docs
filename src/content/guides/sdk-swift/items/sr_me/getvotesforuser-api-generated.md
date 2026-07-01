## Parametri

| Ime | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| urlId | string | query | Da |  |
| userId | string | query | Ne |  |
| anonUserId | string | query | Ne |  |

## Odgovor

Vraća: [`GetVotesForUserResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetVotesForUserResponse.swift)

## Primjer

[inline-code-attrs-start title = 'Primjer getVotesForUser'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Sljedeći uzorci koda su još uvijek beta. Za bilo koji problem, molimo da ga prijavite putem http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 
let userId = "userId_example" // String |  (opcionalno)
let anonUserId = "anonUserId_example" // String |  (opcionalno)

DefaultAPI.getVotesForUser(tenantId: tenantId, urlId: urlId, options: DefaultAPI.GetVotesForUserOptions(userId: userId, anonUserId: anonUserId)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]