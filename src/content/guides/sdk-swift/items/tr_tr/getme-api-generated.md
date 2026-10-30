Identifies the credential in use: the tenant it belongs to and, for OAuth tokens, the user who authorized it.  
Integrations use this to test a connection and label it.

## Parameters

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |

## Response

Döndürür: [`GetMeResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetMeResponse.swift)

## Örnek

[inline-code-attrs-start title = 'getMe Örneği'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Aşağıdaki kod örnekleri hâlâ beta aşamasındadır. Herhangi bir sorun için lütfen http://github.com/OpenAPITools/openapi-generator/issues/new adresine bildirin
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 

DefaultAPI.getMe(tenantId: tenantId) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]