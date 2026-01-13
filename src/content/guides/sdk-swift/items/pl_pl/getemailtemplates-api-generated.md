## Parametry

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Tak |  |
| skip | number | query | Nie |  |

## Odpowiedź

Zwraca: [`GetEmailTemplates200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetEmailTemplates200Response.swift)

## Przykład

[inline-code-attrs-start title = 'Przykład getEmailTemplates'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Poniższe przykłady kodu są nadal w wersji beta. W przypadku problemu, prosimy zgłosić go poprzez http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let skip = 987 // Double |  (opcjonalne)

DefaultAPI.getEmailTemplates(tenantId: tenantId, skip: skip) { (response, error) in
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