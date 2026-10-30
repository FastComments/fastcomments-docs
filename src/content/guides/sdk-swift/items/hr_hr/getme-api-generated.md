Identificira vjerodajnicu u upotrebi: najam kojem pripada i, za OAuth tokena, korisnika koji ju je autorizirao. Integracije koriste ovo za testiranje veze i označavanje.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |

## Response

Returns: [`GetMeResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetMeResponse.swift)

## Example

[inline-code-attrs-start title = 'Primjer getMe'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Sljedeći primjeri koda su još u beta fazi. Za bilo koji problem, molimo prijavite ga putem http://github.com/OpenAPITools/openapi-generator/issues/new
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