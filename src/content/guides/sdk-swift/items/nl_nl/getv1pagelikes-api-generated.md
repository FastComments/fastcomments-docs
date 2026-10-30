## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ja |  |
| urlId | string | query | Ja |  |
| sso | string | query | Nee |  |
| includeUserInfo | string | query | Nee | Indien ingesteld, worden de gebruikers die de pagina hebben geliket geretourneerd (avatars en gebruikersnamen). Alleen "public" wordt geaccepteerd, waardoor privéprofielen worden weggelaten. |

## Respons

Retourneert: [`GetV1PageLikes`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetV1PageLikes.swift)

## Voorbeeld

[inline-code-attrs-start title = 'getV1PageLikes Voorbeeld'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// De volgende codevoorbeelden zijn nog in bèta. Meld eventuele problemen via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 
let sso = "sso_example" // String |  (optioneel)
let includeUserInfo = PageReactIncludeUserInfo() // PageReactIncludeUserInfo | Indien ingesteld, worden de gebruikers die de pagina hebben geliket geretourneerd (avatars en gebruikersnamen). Alleen \"public\" wordt geaccepteerd, waardoor privéprofielen worden weggelaten. (optioneel)

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

---