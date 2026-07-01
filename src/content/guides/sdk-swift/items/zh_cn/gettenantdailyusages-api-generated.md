## 参数

| 名称 | 类型 | 位置 | 必填 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| yearNumber | number | query | 否 |  |
| monthNumber | number | query | 否 |  |
| dayNumber | number | query | 否 |  |
| skip | number | query | 否 |  |

## 响应

返回: [`GetTenantDailyUsagesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetTenantDailyUsagesResponse.swift)

## 示例

[inline-code-attrs-start title = 'getTenantDailyUsages 示例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下代码示例仍为测试版。如有任何问题，请通过 http://github.com/OpenAPITools/openapi-generator/issues/new 报告
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let yearNumber = 987 // Double |  (可选)
let monthNumber = 987 // Double |  (可选)
let dayNumber = 987 // Double |  (可选)
let skip = 987 // Double |  (可选)

DefaultAPI.getTenantDailyUsages(tenantId: tenantId, options: DefaultAPI.GetTenantDailyUsagesOptions(yearNumber: yearNumber, monthNumber: monthNumber, dayNumber: dayNumber, skip: skip)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]