The tenant's batch jobs (imports, exports, page migrations...), newest first.

## Parameters

| 名称 | 类型 | 位置 | 必需 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| type | string | query | 否 | 仅此类型的作业，例如 migrate-pages。 |
| status | string | query | 否 | 仅此状态的作业：requested、running、success 或 failed。 |
| skip | integer | query | 否 | 要跳过的作业数量，用于分页。 |

## Response

Returns: [`GetBatchJobsAPIResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetBatchJobsAPIResponse.swift)

## Example

[inline-code-attrs-start title = 'getBatchJobs 示例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下代码示例仍处于 beta 阶段。如有任何问题，请通过 http://github.com/OpenAPITools/openapi-generator/issues/new 报告
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let type = "type_example" // String | 仅此类型的作业，例如 migrate-pages。（optional）
let status = "status_example" // String | 仅此状态的作业：requested、running、success 或 failed。（optional）
let skip = 987 // Int | 要跳过的作业数量，用于分页。（optional）

DefaultAPI.getBatchJobs(tenantId: tenantId, options: DefaultAPI.GetBatchJobsOptions(type: type, status: status, skip: skip)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]