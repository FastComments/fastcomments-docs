## Parametreler

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes | Denetim günlüklerini okunacak kiracı. |
| limit | integer | query | No | Döndürülecek günlük sayısı. Azami 10000, varsayılan 1000. |
| skip | integer | query | No | Sayfalama için atlanacak günlük sayısı. |
| order | string | query | No | Olay zamanına göre sıralama yönü. Varsayılan DESC (en yeni önce). |
| after | integer | query | No |  |
| before | integer | query | No |  |
| username | string | query | No | Yalnızca bu kullanıcı adıyla gerçekleştirilen olayları döndür. |
| ip | string | query | No | Yalnızca bu IP adresinden gelen olayları döndür. |
| crudType | string | query | No | Yalnızca bu türdeki olayları döndür: c, r, u, d veya giriş. |
| resourceName | string | query | No | Yalnızca bu kaynak için olayları döndür, ör. Kullanıcı veya Moderatör. |
| targetId | string | query | No | Yalnızca etkilenen nesnenin bu kimliğe sahip olduğu olayları döndür. |
| target | string | query | No | Etkilenen nesnenin etiketinde büyük/küçük harfe duyarsız alt dize eşleşmesi, ör. bir kullanıcı adı veya e-posta. |
| includeManagedTenants | boolean | query | No | Ayrıca bu kiracının yönettiği kiracılardan gelen olayları da döndür. |

## Yanıt

Döndürür: [`GetAuditLogsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetAuditLogsResponse.swift)

## Örnek

[inline-code-attrs-start title = 'getAuditLogs Örneği'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Aşağıdaki kod örnekleri hâlâ beta aşamasındadır. Herhangi bir sorun için lütfen http://github.com/OpenAPITools/openapi-generator/issues/new adresinden bildirin.
import FastCommentsSwift

let tenantId = "tenantId_example" // String | Denetim günlüklerini okunacak kiracı.
let limit = 987 // Int | Döndürülecek günlük sayısı. Azami 10000, varsayılan 1000. (opsiyonel)
let skip = 987 // Int | Sayfalama için atlanacak günlük sayısı. (opsiyonel)
let order = SORT_DIR() // SORTDIR | Olay zamanına göre sıralama yönü. Varsayılan DESC (en yeni önce). (opsiyonel)
let after = 987 // Int64 |  (opsiyonel)
let before = 987 // Int64 |  (opsiyonel)
let username = "username_example" // String | Yalnızca bu kullanıcı adıyla gerçekleştirilen olayları döndür. (opsiyonel)
let ip = "ip_example" // String | Yalnızca bu IP adresinden gelen olayları döndür. (opsiyonel)
let crudType = "crudType_example" // String | Yalnızca bu türdeki olayları döndür: c, r, u, d veya giriş. (opsiyonel)
let resourceName = "resourceName_example" // String | Yalnızca bu kaynak için olayları döndür, ör. Kullanıcı veya Moderatör. (opsiyonel)
let targetId = "targetId_example" // String | Yalnızca etkilenen nesnenin bu kimliğe sahip olduğu olayları döndür. (opsiyonel)
let target = "target_example" // String | Etkilenen nesnenin etiketinde büyük/küçük harfe duyarsız alt dize eşleşmesi, ör. bir kullanıcı adı veya e-posta. (opsiyonel)
let includeManagedTenants = true // Bool | Ayrıca bu kiracının yönettiği kiracılardan gelen olayları da döndür. (opsiyonel)

DefaultAPI.getAuditLogs(tenantId: tenantId, options: DefaultAPI.GetAuditLogsOptions(limit: limit, skip: skip, order: order, after: after, before: before, username: username, ip: ip, crudType: crudType, resourceName: resourceName, targetId: targetId, target: target, includeManagedTenants: includeManagedTenants)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]