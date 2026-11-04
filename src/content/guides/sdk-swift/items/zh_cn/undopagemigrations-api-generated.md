将一个批量任务加入队列，该任务会按从新到旧的顺序撤销某个 migrate-pages 任务的每一次迁移。此功能可用 90 天。
迁移之后发生变更的记录不会被改动，因此后续的编辑永远不会被覆盖。
轮询 GET /api/v1/batch-jobs/{batchJobId} 以获取已恢复/已跳过的数量。

## 参数

| 名称 | 类型 | 位置 | 必填 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| undoId | string | path | 是 |  |

## 响应

返回：[`QueuePageMigrationJobAPIResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/QueuePageMigrationJobAPIResponse.swift)

## 示例

[inline-code-attrs-start title = 'undoPageMigrations 示例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
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