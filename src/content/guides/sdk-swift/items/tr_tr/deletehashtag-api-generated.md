## Parametreler

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tag | string | path | Evet |  |
| tenantId | string | query | Hayır |  |

## Yanıt

Döndürür: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/APIEmptyResponse.swift)

## Örnek

[inline-code-attrs-start title = 'deleteHashTag Example'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Aşağıdaki kod örnekleri hâlâ beta aşamasındadır. Herhangi bir sorun için lütfen http://github.com/OpenAPITools/openapi-generator/issues/new adresi üzerinden bildirin
import FastCommentsSwift

let tag = "tag_example" // String | 
let tenantId = "tenantId_example" // String |  (isteğe bağlı)
let deleteHashTagRequestBody = DeleteHashTagRequestBody(tenantId: "tenantId_example") // DeleteHashTagRequestBody |  (isteğe bağlı)

DefaultAPI.deleteHashTag(tag: tag, tenantId: tenantId, deleteHashTagRequestBody: deleteHashTagRequestBody) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]