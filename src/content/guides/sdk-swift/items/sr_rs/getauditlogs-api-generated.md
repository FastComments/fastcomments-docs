## Параметри

| Име | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes | Тенант за који се читају аудиторски записи. |
| limit | integer | query | No | Колико записа да се врати. Максимум 10000, подразумевано 1000. |
| skip | integer | query | No | Колико записа да се прескочи, за пагинацију. |
| order | string | query | No | Смер сортирања по времену догађаја. Подразумевано DESC (најновији први). |
| after | integer | query | No |  |
| before | integer | query | No |  |
| username | string | query | No | Враћа само догађаје извршене од стране овог корисничког имена. |
| ip | string | query | No | Враћа само догађаје са ове IP адресе. |
| crudType | string | query | No | Враћа само догађаје овог типа: c, r, u, d, или login. |
| resourceName | string | query | No | Враћа само догађаје за овај ресурс, нпр. User или Moderator. |
| targetId | string | query | No | Враћа само догађаје чији је утицани објекат има овај ID. |
| target | string | query | No | Поклапање подстринга без разликовања величине слова на ознаци утицаног објекта, нпр. корисничко име или е‑mail. |
| includeManagedTenants | boolean | query | No | Такође враћа догађаје од тенаната које овај тенант управља. |

## Одговор

Враћа: [`GetAuditLogsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetAuditLogsResponse.swift)

## Пример

[inline-code-attrs-start title = 'Primer getAuditLogs'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следећи пример кода је још у бета фази. За било који проблем, молимо пријавите га на http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | Тенант за који се читају аудиторски записи.
let limit = 987 // Int | Колико записа да се врати. Максимум 10000, подразумевано 1000. (optional)
let skip = 987 // Int | Колико записа да се прескочи, за пагинацију. (optional)
let order = SORT_DIR() // SORTDIR | Смер сортирања по времену догађаја. Подразумевано DESC (најновији први). (optional)
let after = 987 // Int64 |  (optional)
let before = 987 // Int64 |  (optional)
let username = "username_example" // String | Враћа само догађаје извршене од стране овог корисничког имена. (optional)
let ip = "ip_example" // String | Враћа само догађаје са ове IP адресе. (optional)
let crudType = "crudType_example" // String | Враћа само догађаје овог типа: c, r, u, d, или login. (optional)
let resourceName = "resourceName_example" // String | Враћа само догађаје за овај ресурс, нпр. User или Moderator. (optional)
let targetId = "targetId_example" // String | Враћа само догађаје чији је утицани објекат има овај ID. (optional)
let target = "target_example" // String | Поклапање подстринга без разликовања величине слова на ознаци утицаног објекта, нпр. корисничко име или е‑mail. (optional)
let includeManagedTenants = true // Bool | Такође враћа догађаје од тенаната које овај тенант управља. (optional)

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