## Parametreler

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |
| domain | string | path | Evet |  |

## Yanıt

Döndürür: [`DeleteDomainConfigResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/DeleteDomainConfigResponse.swift)

## Örnek

[inline-code-attrs-start title = 'deleteDomainConfig Örneği'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Aşağıdaki kod örnekleri hala beta aşamasındadır. Herhangi bir sorun için lütfen http://github.com/OpenAPITools/openapi-generator/issues/new adresi üzerinden bildirin
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let domain = "domain_example" // String | 

DefaultAPI.deleteDomainConfig(tenantId: tenantId, domain: domain) { (response, error) in
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