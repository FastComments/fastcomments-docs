Прошли коментатори на страници који тренутно НИСУ online. Сортирано по displayName.
Користите ово након што исцрпите /users/online да рендерујете секцију "Members".
Cursor пагинација по commenterName: сервер пролази делимични индекс {tenantId, urlId, commenterName}
од afterName напријед преко $gt, без трошка $skip.

## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Identifikator URL stranice (očišćen na serverskoj strani). |
| afterName | string | query | No | Kursor: proslijedite nextAfterName iz prethodnog odgovora. |
| afterUserId | string | query | No | Kursor za razrješenje: proslijedite nextAfterUserId iz prethodnog odgovora. Obavezno kada je afterName postavljen kako bi se pri istim imenima unosi ne bi izgubili. |

## Одговор

Vraća: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersOfflineResponse.swift)

## Примјер

[inline-code-attrs-start title = 'getOfflineUsers Primjer'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Sljedeći primjeri koda su još u beta fazi. Za bilo koji problem, prijavite ga putem http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | Identifikator URL stranice (očišćen na serverskoj strani).
let afterName = "afterName_example" // String | Kursor: proslijedite nextAfterName iz prethodnog odgovora. (opcionalno)
let afterUserId = "afterUserId_example" // String | Kursor za razrješenje: proslijedite nextAfterUserId iz prethodnog odgovora. Obavezno kada je afterName postavljen kako bi se pri istim imenima unosi ne bi izgubili. (opcionalno)

PublicAPI.getOfflineUsers(tenantId: tenantId, urlId: urlId, afterName: afterName, afterUserId: afterUserId) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]