## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|---------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes |  |
| sso | string | query | No |  |
| includeUserInfo | string | query | No | Wanneer ingesteld, retourneert de gebruikers voor elke react-id (avatars en gebruikersnamen). Alleen "public" wordt geaccepteerd, wat privéprofielen weglaten betekent. |

## Response

Returns: [`GetV2PageReacts`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetV2PageReacts.swift)

## Example

[inline-code-attrs-start title = 'getV2PageReacts Voorbeeld'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// De volgende codevoorbeelden zijn nog in bèta. Meld eventuele problemen via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 
let sso = "sso_example" // String |  (optioneel)
let includeUserInfo = PageReactIncludeUserInfo() // PageReactIncludeUserInfo | Wanneer ingesteld, retourneert de gebruikers voor elke react-id (avatars en gebruikersnamen). Alleen \"public\" wordt geaccepteerd, wat privéprofielen weglaten betekent. (optioneel)

PublicAPI.getV2PageReacts(tenantId: tenantId, urlId: urlId, options: PublicAPI.GetV2PageReactsOptions(sso: sso, includeUserInfo: includeUserInfo)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]