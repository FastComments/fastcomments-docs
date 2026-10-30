Queues bir toplu işi kuyruğa alır; bu iş, bir migrate‑pages işinden gelen tüm göçleri tersine çevirir, en yenisi önce. 90 gün boyunca kullanılabilir.  
Göçten bu yana değişen kayıtlar olduğu gibi bırakılır, böylece daha sonraki bir düzenleme asla üzerine yazılmaz.  
`GET /api/v1/batch-jobs/{batchJobId}` isteğiyle geri yüklenen/atlanan sayıları sorgulayın.

## Parameters

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |
| undoId | string | path | Evet |  |

## Response

Returns: [`QueuePageMigrationJobAPIResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/QueuePageMigrationJobAPIResponse.swift)

## Example

[inline-code-attrs-start title = 'undoPageMigrations Örneği'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Aşağıdaki kod örnekleri hâlâ beta aşamasındadır. Herhangi bir sorun için lütfen http://github.com/OpenAPITools/openapi-generator/issues/new adresinden bildirin
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let undoId = "undoId_example" // String | 

DefaultAPI.undoPageMigrations(tenantId: tenantId, undoId: undoId) { (response, error) in
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