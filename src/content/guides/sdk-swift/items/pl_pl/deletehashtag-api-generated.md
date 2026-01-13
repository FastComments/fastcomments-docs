## Parametry

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|----------|----------|-------------|
| tag | string | path | Tak |  |
| tenantId | string | query | Nie |  |

## Odpowiedź

Zwraca: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/FlagCommentPublic200Response.swift)

## Przykład

[inline-code-attrs-start title = 'Przykład deleteHashTag'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Poniższe przykłady kodu są nadal w wersji beta. W razie problemów zgłoś je pod adresem http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tag = "tag_example" // String | 
let tenantId = "tenantId_example" // String |  (opcjonalne)
let deleteHashTagRequest = DeleteHashTag_request(tenantId: "tenantId_example") // DeleteHashTagRequest |  (opcjonalne)

DefaultAPI.deleteHashTag(tag: tag, tenantId: tenantId, deleteHashTagRequest: deleteHashTagRequest) { (response, error) in
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