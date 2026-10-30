## Параметри

| Име | Тип | Местоположение | Задължително | Описание |
|------|------|----------------|--------------|----------|
| tenantId | string | query | Yes | Наемателят, за който се четат журналите за одит. |
| limit | integer | query | No | Колко журнала да се върнат. Максимум 10000, по подразбиране 1000. |
| skip | integer | query | No | Колко журнала да се пропуснат, за странициране. |
| order | string | query | No | Посока на сортиране по времето на събитието. По подразбиране DESC (най-новите първи). |
| after | integer | query | No |  |
| before | integer | query | No |  |
| username | string | query | No | Връща само събития, извършени от това потребителско име. |
| ip | string | query | No | Връща само събития от този IP адрес. |
| crudType | string | query | No | Връща само събития от този тип: c, r, u, d, или login. |
| resourceName | string | query | No | Връща само събития за този ресурс, напр. User или Moderator. |
| targetId | string | query | No | Връща само събития, чиито засегнат обект има това id. |
| target | string | query | No | Съвпадение на подниз без значение за регистъра върху етикета на засегнатия обект, напр. username или email. |
| includeManagedTenants | boolean | query | No | Също така връща събития от наематели, които този наемател управлява. |

## Отговор

Връща: [`GetAuditLogsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetAuditLogsResponse.swift)

## Пример

[inline-code-attrs-start title = 'getAuditLogs Пример'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следващите примерни кодове все още са бета. За всеки проблем, моля докладвайте чрез http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | Наемателят, за който се четат журналите за одит.
let limit = 987 // Int | Колко журнала да се върнат. Максимум 10000, по подразбиране 1000. (по избор)
let skip = 987 // Int | Колко журнала да се пропуснат, за странициране. (по избор)
let order = SORT_DIR() // SORTDIR | Посока на сортиране по времето на събитието. По подразбиране DESC (най-новите първи). (по избор)
let after = 987 // Int64 |  (по избор)
let before = 987 // Int64 |  (по избор)
let username = "username_example" // String | Връща само събития, извършени от това потребителско име. (по избор)
let ip = "ip_example" // String | Връща само събития от този IP адрес. (по избор)
let crudType = "crudType_example" // String | Връща само събития от този тип: c, r, u, d, или login. (по избор)
let resourceName = "resourceName_example" // String | Връща само събития за този ресурс, напр. User или Moderator. (по избор)
let targetId = "targetId_example" // String | Връща само събития, чиито засегнат обект има това id. (по избор)
let target = "target_example" // String | Съвпадение на подниз без значение за регистъра върху етикета на засегнатия обект, напр. username или email. (по избор)
let includeManagedTenants = true // Bool | Също така връща събития от наематели, които този наемател управлява. (по избор)

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