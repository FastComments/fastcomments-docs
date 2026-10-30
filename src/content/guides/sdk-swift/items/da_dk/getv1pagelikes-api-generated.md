## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes |  |
| sso | string | query | Yes |  |
| includeUserInfo | string | query | No | Når den er angivet, returnerer den brugerne, der har liket siden (avatarer og brugernavne). Kun \"public\" accepteres, hvilket udelader private profiler. |

## Svar

Returnerer: [`GetV1PageLikes`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetV1PageLikes.swift)

## Eksempel

[inline-code-attrs-start title = 'getV1PageLikes Eksempel'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Følgende kodeeksempler er stadig i beta. For eventuelle problemer, rapporter venligst via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 
let sso = "sso_example" // String |  (valgfri)
let includeUserInfo = PageReactIncludeUserInfo() // PageReactIncludeUserInfo | Når den er angivet, returnerer den brugerne, der har liket siden (avatarer og brugernavne). Kun \"public\" accepteres, hvilket udelader private profiler. (valgfri)

PublicAPI.getV1PageLikes(tenantId: tenantId, urlId: urlId, options: PublicAPI.GetV1PageLikesOptions(sso: sso, includeUserInfo: includeUserInfo)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]