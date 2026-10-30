---
租戶的批次工作（匯入、匯出、頁面遷移...），依最新排序。

## 參數

| 名稱 | 類型 | 位置 | 必填 | 說明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| type | string | query | 否 | 僅此類型的工作，例如 migrate-pages。 |
| status | string | query | 否 | 僅此狀態的工作：requested、running、success 或 failed。 |
| skip | integer | query | 否 | 要跳過多少工作，用於分頁。 |

## 回應

返回：[`GetBatchJobsAPIResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetBatchJobsAPIResponse.swift)

## 範例

[inline-code-attrs-start title = '取得批次工作 範例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下程式碼範例仍屬測試版。若有任何問題，請透過 http://github.com/OpenAPITools/openapi-generator/issues/new 回報
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let type = "type_example" // String | 僅此類型的工作，例如 migrate-pages。（可選）
let status = "status_example" // String | 僅此狀態的工作：requested、running、success 或 failed。（可選）
let skip = 987 // Int | 要跳過多少工作，用於分頁。（可選）

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

---