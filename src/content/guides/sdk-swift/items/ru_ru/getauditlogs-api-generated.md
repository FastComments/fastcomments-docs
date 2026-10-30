## Параметры

| Имя | Тип | Расположение | Обязательно | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes | Тенант, для которого читаются журналы аудита. |
| limit | integer | query | No | Количество журналов для возврата. Максимум 10000, по умолчанию 1000. |
| skip | integer | query | No | Сколько журналов пропустить, для постраничного вывода. |
| order | string | query | No | Направление сортировки по времени события. По умолчанию DESC (сначала новые). |
| after | integer | query | No |  |
| before | integer | query | No |  |
| username | string | query | No | Возвращать только события, выполненные этим именем пользователя. |
| ip | string | query | No | Возвращать только события с этого IP-адреса. |
| crudType | string | query | No | Возвращать только события этого типа: c, r, u, d или login. |
| resourceName | string | query | No | Возвращать только события для этого ресурса, например User или Moderator. |
| targetId | string | query | No | Возвращать только события, у которых затронутый объект имеет этот идентификатор. |
| target | string | query | No | Поиск подстроки без учёта регистра в метке затронутого объекта, например имя пользователя или email. |
| includeManagedTenants | boolean | query | No | Также возвращать события от тенантов, которыми управляет данный тенант. |

## Ответ

Возвращает: [`GetAuditLogsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetAuditLogsResponse.swift)

## Пример

[inline-code-attrs-start title = 'Пример getAuditLogs'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следующие образцы кода находятся в бета-версии. При возникновении проблем, пожалуйста, сообщайте по адресу http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | Тенант, для которого читаются журналы аудита.
let limit = 987 // Int | Количество журналов для возврата. Максимум 10000, по умолчанию 1000. (необязательно)
let skip = 987 // Int | Сколько журналов пропустить, для постраничного вывода. (необязательно)
let order = SORT_DIR() // SORTDIR | Направление сортировки по времени события. По умолчанию DESC (сначала новые). (необязательно)
let after = 987 // Int64 |  (необязательно)
let before = 987 // Int64 |  (необязательно)
let username = "username_example" // String | Возвращать только события, выполненные этим именем пользователя. (необязательно)
let ip = "ip_example" // String | Возвращать только события с этого IP-адреса. (необязательно)
let crudType = "crudType_example" // String | Возвращать только события этого типа: c, r, u, d или login. (необязательно)
let resourceName = "resourceName_example" // String | Возвращать только события для этого ресурса, например User или Moderator. (необязательно)
let targetId = "targetId_example" // String | Возвращать только события, у которых затронутый объект имеет этот идентификатор. (необязательно)
let target = "target_example" // String | Поиск подстроки без учёта регистра в метке затронутого объекта, например имя пользователя или email. (необязательно)
let includeManagedTenants = true // Bool | Также возвращать события от тенантов, которыми управляет данный тенант. (необязательно)

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