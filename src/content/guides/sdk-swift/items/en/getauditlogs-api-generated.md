## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes | The tenant to read audit logs for. |
| limit | integer | query | No | How many logs to return. Max 10000, defaults to 1000. |
| skip | integer | query | No | How many logs to skip, for paging. |
| order | string | query | No | Sort direction on the event time. Defaults to DESC (newest first). |
| after | integer | query | No |  |
| before | integer | query | No |  |
| username | string | query | No | Only return events performed by this username. |
| ip | string | query | No | Only return events from this IP address. |
| crudType | string | query | No | Only return events of this type: c, r, u, d, or login. |
| resourceName | string | query | No | Only return events for this resource, e.g. User or Moderator. |
| targetId | string | query | No | Only return events whose affected object has this id. |
| target | string | query | No | Case-insensitive substring match on the affected object's label, e.g. a username or email. |
| includeManagedTenants | boolean | query | No | Also return events from tenants this tenant manages. |

## Response

Returns: [`GetAuditLogsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetAuditLogsResponse.swift)

## Example

[inline-code-attrs-start title = 'getAuditLogs Example'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | The tenant to read audit logs for.
let limit = 987 // Int | How many logs to return. Max 10000, defaults to 1000. (optional)
let skip = 987 // Int | How many logs to skip, for paging. (optional)
let order = SORT_DIR() // SORTDIR | Sort direction on the event time. Defaults to DESC (newest first). (optional)
let after = 987 // Int64 |  (optional)
let before = 987 // Int64 |  (optional)
let username = "username_example" // String | Only return events performed by this username. (optional)
let ip = "ip_example" // String | Only return events from this IP address. (optional)
let crudType = "crudType_example" // String | Only return events of this type: c, r, u, d, or login. (optional)
let resourceName = "resourceName_example" // String | Only return events for this resource, e.g. User or Moderator. (optional)
let targetId = "targetId_example" // String | Only return events whose affected object has this id. (optional)
let target = "target_example" // String | Case-insensitive substring match on the affected object's label, e.g. a username or email. (optional)
let includeManagedTenants = true // Bool | Also return events from tenants this tenant manages. (optional)

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
