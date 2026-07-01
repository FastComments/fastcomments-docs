Past komentatori na stranici koji NIJE trenutno online. Sortirano po displayName.  
Koristite ovo nakon što ste iscrpili /users/online za prikaz sekcije „Članovi”.  
Kursor paginacija po commenterName: server prolazi kroz djelomični {tenantId, urlId, commenterName} indeks od afterName naprijed putem $gt, bez troška $skip.  

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Identifikator URL-a stranice (čišćen na poslužitelju). |
| afterName | string | query | No | Kursor: proslijedite nextAfterName iz prethodnog odgovora. |
| afterUserId | string | query | No | Kursor razrješivač neriješenosti: proslijedite nextAfterUserId iz prethodnog odgovora. Potrebno kada je afterName postavljen kako se ne bi propuštali unosi zbog vezivanja imena. |

## Response

Returns: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersOfflineResponse.swift)

## Example

[inline-code-attrs-start title = 'Primjer getOfflineUsers'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Sljedeći primjeri koda još su u beta fazi. Za bilo koji problem, molimo prijavite ga putem http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | Identifikator URL-a stranice (čišćen na poslužitelju).
let afterName = "afterName_example" // String | Kursor: proslijedite nextAfterName iz prethodnog odgovora. (opcijski)
let afterUserId = "afterUserId_example" // String | Kursor razrješivač neriješenosti: proslijedite nextAfterUserId iz prethodnog odgovora. Potrebno kada je afterName postavljen kako se ne bi propuštali unosi zbog vezivanja imena. (opcijski)

PublicAPI.getOfflineUsers(tenantId: tenantId, urlId: urlId, options: PublicAPI.GetOfflineUsersOptions(afterName: afterName, afterUserId: afterUserId)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]