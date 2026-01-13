## 参数

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| yearNumber | number | query | No |  |
| monthNumber | number | query | No |  |
| dayNumber | number | query | No |  |
| skip | number | query | No |  |

## 响应

返回：[`GetTenantDailyUsages200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetTenantDailyUsages200Response.swift)

## 示例

[inline-code-attrs-start title = 'getTenantDailyUsages 示例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下代码示例仍处于测试阶段。如有任何问题，请通过 http://github.com/OpenAPITools/openapi-generator/issues/new 报告
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let yearNumber = 987 // Double |  (可选)
let monthNumber = 987 // Double |  (可选)
let dayNumber = 987 // Double |  (可选)
let skip = 987 // Double |  (可选)

DefaultAPI.getTenantDailyUsages(tenantId: tenantId, yearNumber: yearNumber, monthNumber: monthNumber, dayNumber: dayNumber, skip: skip) { (response, error) in
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