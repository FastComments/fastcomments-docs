## Parametreler

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |
| userId | string | query | Hayır |  |
| limit | number | query | Hayır |  |
| skip | number | query | Hayır |  |

## Yanıt

Döndürür: [`GetUserBadgeProgressList200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetUserBadgeProgressList200Response.swift)

## Örnek

[inline-code-attrs-start title = 'getUserBadgeProgressList Örneği'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Aşağıdaki kod örnekleri hâlâ beta aşamasındadır. Herhangi bir sorun için lütfen http://github.com/OpenAPITools/openapi-generator/issues/new üzerinden raporlayın
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let userId = "userId_example" // String |  (isteğe bağlı)
let limit = 987 // Double |  (isteğe bağlı)
let skip = 987 // Double |  (isteğe bağlı)

DefaultAPI.getUserBadgeProgressList(tenantId: tenantId, userId: userId, limit: limit, skip: skip) { (response, error) in
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