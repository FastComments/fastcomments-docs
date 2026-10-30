## Параметри

| Назва | Тип | Розташування | Обов’язковий | Опис |
|------|------|--------------|--------------|------|
| tenantId | string | query | Yes | Орендар, для якого читаються журнали аудиту. |
| limit | integer | query | No | Кількість журналів, які повернути. Максимум 10000, за замовчуванням 1000. |
| skip | integer | query | No | Кількість журналів, які пропустити, для пагінації. |
| order | string | query | No | Напрямок сортування за часом події. За замовчуванням DESC (найновіші першими). |
| after | integer | query | No |  |
| before | integer | query | No |  |
| username | string | query | No | Повернути лише події, виконані цим ім'ям користувача. |
| ip | string | query | No | Повернути лише події з цієї IP-адреси. |
| crudType | string | query | No | Повернути лише події цього типу: c, r, u, d або login. |
| resourceName | string | query | No | Повернути лише події для цього ресурсу, напр., User або Moderator. |
| targetId | string | query | No | Повернути лише події, у яких уражений об’єкт має цей ідентифікатор. |
| target | string | query | No | Пошук підрядка без урахування регістру в мітці ураженого об’єкта, напр., ім'я користувача або електронна пошта. |
| includeManagedTenants | boolean | query | No | Також повернути події від орендарів, якими керує цей орендар. |

## Відповідь

Повертає: [`GetAuditLogsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetAuditLogsResponse.swift)

## Приклад

[inline-code-attrs-start title = 'getAuditLogs Приклад'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Наступні приклади коду ще у бета-версії. У разі будь-яких проблем, будь ласка, повідомте за адресою http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | Орендар, для якого читаються журнали аудиту.
let limit = 987 // Int | Кількість журналів, які повернути. Максимум 10000, за замовчуванням 1000. (optional)
let skip = 987 // Int | Кількість журналів, які пропустити, для пагінації. (optional)
let order = SORT_DIR() // SORTDIR | Напрямок сортування за часом події. За замовчуванням DESC (найновіші першими). (optional)
let after = 987 // Int64 |  (optional)
let before = 987 // Int64 |  (optional)
let username = "username_example" // String | Повернути лише події, виконані цим ім'ям користувача. (optional)
let ip = "ip_example" // String | Повернути лише події з цієї IP-адреси. (optional)
let crudType = "crudType_example" // String | Повернути лише події цього типу: c, r, u, d або login. (optional)
let resourceName = "resourceName_example" // String | Повернути лише події для цього ресурсу, напр., User або Moderator. (optional)
let targetId = "targetId_example" // String | Повернути лише події, у яких уражений об’єкт має цей ідентифікатор. (optional)
let target = "target_example" // String | Пошук підрядка без урахування регістру в мітці ураженого об’єкта, напр., ім'я користувача або електронна пошта. (optional)
let includeManagedTenants = true // Bool | Також повернути події від орендарів, якими керує цей орендар. (optional)

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